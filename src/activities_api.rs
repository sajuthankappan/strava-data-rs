use std::sync::Arc;

use log::debug;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;

use crate::configuration::Configuration;
use crate::error::Error;
use crate::models::DetailedActivity;

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

    pub async fn get_activity_by_id(
        &self,
        id: i64,
        access_token: &str,
    ) -> Result<Option<DetailedActivity>, Error> {
        debug!("get_activity_by_id {}", id);
        let url = format!("{}/activities/{id}", self.configuration.base_path, id = id);
        match self.get(&url, &[], access_token).await {
            Ok(activity) => Ok(Some(activity)),
            Err(Error::Status { status: 404, .. }) => {
                log::warn!("activity {} not found", id);
                Ok(None)
            }
            Err(e) => Err(e),
        }
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
    ) -> Result<Vec<DetailedActivity>, Error> {
        debug!("get_logged_in_athlete_activities");
        let url = format!("{}/athlete/activities", self.configuration.base_path);
        let mut query = vec![("page", i64::from(page)), ("per_page", i64::from(per_page))];
        if let Some(before) = before {
            query.push(("before", before));
        }
        if let Some(after) = after {
            query.push(("after", after));
        }
        self.get(&url, &query, access_token).await
    }

    async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        query: &[(&str, i64)],
        access_token: &str,
    ) -> Result<T, Error> {
        let res = self
            .client
            .get(url)
            .query(query)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        let status = res.status();
        if status != StatusCode::OK {
            let body = res.text().await.unwrap_or_default();
            return Err(Error::from_status(status, body));
        }

        let bytes = res.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path, query_param, query_param_is_missing};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn api_for(server: &MockServer) -> ActivitiesApi {
        ActivitiesApi::new(Arc::new(Configuration {
            base_path: server.uri(),
        }))
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
            .unwrap();
        assert_eq!(activity.id, Some(1));
    }

    #[tokio::test]
    async fn returns_none_when_not_found() {
        let server = MockServer::start().await;
        mock_activity_response(&server, ResponseTemplate::new(404)).await;

        let activity = api_for(&server)
            .get_activity_by_id(1, "token")
            .await
            .unwrap();
        assert!(activity.is_none());
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
        mock_activity_response(&server, ResponseTemplate::new(429)).await;

        let error = api_for(&server)
            .get_activity_by_id(1, "token")
            .await
            .unwrap_err();
        assert!(matches!(error, Error::RateLimited { .. }));
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
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"[{"id": 1}]"#))
            .mount(&server)
            .await;

        let activities = api_for(&server)
            .get_logged_in_athlete_activities(Some(1767225600), Some(1735689600), 2, 30, "token")
            .await
            .unwrap();
        assert_eq!(activities.len(), 1);
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
        assert!(activities.is_empty());
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
