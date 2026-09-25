use std::sync::Arc;

use log::debug;
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::configuration::Configuration;
use crate::error::Error;
use crate::models::{DetailedActivity, SummaryActivity};
use crate::rate_limit::{ApiResponse, RateLimit};

pub struct ActivitiesApi {
    configuration: Arc<Configuration>,
    client: Client,
}

impl ActivitiesApi {
    pub fn new(configuration: Arc<Configuration>) -> ActivitiesApi {
        ActivitiesApi {
            configuration,
            client: Client::new(),
        }
    }

    /// Get an activity, or `None` if it does not exist
    pub async fn get_activity_by_id(
        &self,
        id: i64,
        access_token: &str,
    ) -> Result<ApiResponse<Option<DetailedActivity>>, Error> {
        debug!("get_activity_by_id {}", id);
        let url = format!("{}/activities/{id}", self.configuration.base_path, id = id);
        let (res, rate_limit) = self.send(&url, &[], access_token).await?;
        if res.status() == StatusCode::NOT_FOUND {
            log::warn!("activity {} not found", id);
            return Ok(ApiResponse {
                data: None,
                rate_limit,
            });
        }
        let activity = Self::decode(res, rate_limit).await?;
        Ok(ApiResponse {
            data: Some(activity),
            rate_limit,
        })
    }

    /// List the logged-in athlete's activities, optionally limited to those started
    /// before and/or after the given Unix timestamps (in seconds)
    pub async fn get_logged_in_athlete_activities(
        &self,
        before: Option<i64>,
        after: Option<i64>,
        page: i32,
        per_page: i32,
        access_token: &str,
    ) -> Result<ApiResponse<Vec<SummaryActivity>>, Error> {
        debug!("get_logged_in_athlete_activities");
        let url = format!("{}/athlete/activities", self.configuration.base_path);
        let mut query = vec![("page", i64::from(page)), ("per_page", i64::from(per_page))];
        if let Some(before) = before {
            query.push(("before", before));
        }
        if let Some(after) = after {
            query.push(("after", after));
        }
        let (res, rate_limit) = self.send(&url, &query, access_token).await?;
        let activities = Self::decode(res, rate_limit).await?;
        Ok(ApiResponse {
            data: activities,
            rate_limit,
        })
    }

    async fn send(
        &self,
        url: &str,
        query: &[(&str, i64)],
        access_token: &str,
    ) -> Result<(Response, Option<RateLimit>), Error> {
        let res = self
            .client
            .get(url)
            .query(query)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;
        let rate_limit = RateLimit::from_headers(res.headers());
        Ok((res, rate_limit))
    }

    async fn decode<T: DeserializeOwned>(
        res: Response,
        rate_limit: Option<RateLimit>,
    ) -> Result<T, Error> {
        let status = res.status();
        if status != StatusCode::OK {
            let body = res.text().await.unwrap_or_default();
            return Err(Error::from_status(status, body, rate_limit));
        }

        let bytes = res.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rate_limit::RateLimitWindow;
    use wiremock::matchers::{header, method, path, query_param, query_param_is_missing};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn api_for(server: &MockServer) -> ActivitiesApi {
        ActivitiesApi::new(Arc::new(Configuration {
            base_path: server.uri(),
        }))
    }

    fn with_rate_limit_headers(response: ResponseTemplate) -> ResponseTemplate {
        response
            .insert_header("X-RateLimit-Limit", "200,2000")
            .insert_header("X-RateLimit-Usage", "20,300")
            .insert_header("X-ReadRateLimit-Limit", "100,1000")
            .insert_header("X-ReadRateLimit-Usage", "10,150")
    }

    fn expected_rate_limit() -> RateLimit {
        RateLimit {
            overall: Some(RateLimitWindow {
                short_term_limit: 200,
                daily_limit: 2000,
                short_term_usage: 20,
                daily_usage: 300,
            }),
            read: Some(RateLimitWindow {
                short_term_limit: 100,
                daily_limit: 1000,
                short_term_usage: 10,
                daily_usage: 150,
            }),
        }
    }

    async fn mock_activity_response(server: &MockServer, response: ResponseTemplate) {
        Mock::given(method("GET"))
            .and(path("/activities/1"))
            .and(header("Authorization", "Bearer token"))
            .respond_with(response)
            .mount(server)
            .await;
    }

    #[tokio::test]
    async fn returns_activity() {
        let server = MockServer::start().await;
        mock_activity_response(
            &server,
            ResponseTemplate::new(200).set_body_string(r#"{"id": 1, "type": "Run"}"#),
        )
        .await;

        let activity = api_for(&server)
            .get_activity_by_id(1, "token")
            .await
            .unwrap()
            .data
            .unwrap();
        assert_eq!(activity.id, Some(1));
    }

    #[tokio::test]
    async fn returns_rate_limit_with_activity() {
        let server = MockServer::start().await;
        mock_activity_response(
            &server,
            with_rate_limit_headers(ResponseTemplate::new(200).set_body_string(r#"{"id": 1}"#)),
        )
        .await;

        let response = api_for(&server)
            .get_activity_by_id(1, "token")
            .await
            .unwrap();
        assert_eq!(response.rate_limit, Some(expected_rate_limit()));
    }

    #[tokio::test]
    async fn returns_no_rate_limit_without_headers() {
        let server = MockServer::start().await;
        mock_activity_response(
            &server,
            ResponseTemplate::new(200).set_body_string(r#"{"id": 1}"#),
        )
        .await;

        let response = api_for(&server)
            .get_activity_by_id(1, "token")
            .await
            .unwrap();
        assert!(response.rate_limit.is_none());
    }

    #[tokio::test]
    async fn returns_rate_limit_when_not_found() {
        let server = MockServer::start().await;
        mock_activity_response(&server, with_rate_limit_headers(ResponseTemplate::new(404))).await;

        let response = api_for(&server)
            .get_activity_by_id(1, "token")
            .await
            .unwrap();
        assert!(response.data.is_none());
        assert_eq!(response.rate_limit, Some(expected_rate_limit()));
    }

    #[tokio::test]
    async fn returns_none_when_not_found() {
        let server = MockServer::start().await;
        mock_activity_response(&server, ResponseTemplate::new(404)).await;

        let activity = api_for(&server)
            .get_activity_by_id(1, "token")
            .await
            .unwrap();
        assert!(activity.data.is_none());
    }

    #[tokio::test]
    async fn maps_unauthorized() {
        let server = MockServer::start().await;
        mock_activity_response(
            &server,
            ResponseTemplate::new(401).set_body_string("expired"),
        )
        .await;

        let error = api_for(&server)
            .get_activity_by_id(1, "token")
            .await
            .unwrap_err();
        assert!(matches!(error, Error::Unauthorized { body } if body == "expired"));
    }

    #[tokio::test]
    async fn maps_rate_limited() {
        let server = MockServer::start().await;
        mock_activity_response(
            &server,
            with_rate_limit_headers(ResponseTemplate::new(429).set_body_string("slow down")),
        )
        .await;

        let error = api_for(&server)
            .get_activity_by_id(1, "token")
            .await
            .unwrap_err();
        let Error::RateLimited { body, rate_limit } = error else {
            panic!("expected RateLimited, got {error:?}");
        };
        assert_eq!(body, "slow down");
        assert_eq!(rate_limit, Some(expected_rate_limit()));
    }

    #[tokio::test]
    async fn maps_other_status() {
        let server = MockServer::start().await;
        mock_activity_response(&server, ResponseTemplate::new(500).set_body_string("oops")).await;

        let error = api_for(&server)
            .get_activity_by_id(1, "token")
            .await
            .unwrap_err();
        assert!(matches!(error, Error::Status { status: 500, body } if body == "oops"));
    }

    #[tokio::test]
    async fn maps_decode_failure() {
        let server = MockServer::start().await;
        mock_activity_response(
            &server,
            ResponseTemplate::new(200).set_body_string(r#"{"type": "NotAnActivity"}"#),
        )
        .await;

        let error = api_for(&server)
            .get_activity_by_id(1, "token")
            .await
            .unwrap_err();
        assert!(matches!(error, Error::Decode(_)));
    }

    #[tokio::test]
    async fn sends_before_and_after_when_present() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/athlete/activities"))
            .and(query_param("before", "1767225600"))
            .and(query_param("after", "1735689600"))
            .and(query_param("page", "2"))
            .and(query_param("per_page", "30"))
            .respond_with(with_rate_limit_headers(
                ResponseTemplate::new(200).set_body_string(r#"[{"id": 1}]"#),
            ))
            .mount(&server)
            .await;

        let activities = api_for(&server)
            .get_logged_in_athlete_activities(Some(1767225600), Some(1735689600), 2, 30, "token")
            .await
            .unwrap();
        assert_eq!(activities.data.len(), 1);
        assert_eq!(activities.rate_limit, Some(expected_rate_limit()));
    }

    #[tokio::test]
    async fn omits_before_and_after_when_absent() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/athlete/activities"))
            .and(query_param_is_missing("before"))
            .and(query_param_is_missing("after"))
            .and(query_param("page", "1"))
            .and(query_param("per_page", "30"))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&server)
            .await;

        let activities = api_for(&server)
            .get_logged_in_athlete_activities(None, None, 1, 30, "token")
            .await
            .unwrap();
        assert!(activities.data.is_empty());
    }

    #[tokio::test]
    async fn maps_request_failure() {
        let api = ActivitiesApi::new(Arc::new(Configuration {
            base_path: String::from("http://127.0.0.1:1"),
        }));

        let error = api.get_activity_by_id(1, "token").await.unwrap_err();
        assert!(matches!(error, Error::Request(_)));
    }
}
