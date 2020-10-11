use log::debug;
use reqwest::{Client, StatusCode};
use std::rc::Rc;

use crate::configuration::Configuration;
use crate::models::DetailedActivity;

pub struct ActivitiesApi {
    configuration: Rc<Configuration>,
}

impl ActivitiesApi {
    pub fn new(configuration: Rc<Configuration>) -> ActivitiesApi {
        ActivitiesApi {
            configuration: configuration,
        }
    }

    pub async fn get_activity_by_id(
        &self,
        id: i64,
        access_token: &String,
    ) -> Result<Option<DetailedActivity>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        debug!("get_activity_by_id {}", id);
        let url = format!("{}/activities/{id}", self.configuration.base_path, id = id);
        let authorization = format!("Bearer {}", access_token);
        let res = Client::new()
            .get(url.as_str())
            .header("Authorization", authorization)
            .send()
            .await?;

        if res.status().clone() != StatusCode::OK {
            if (res.status().clone()) == StatusCode::NOT_FOUND {
                log::warn!("activity {} not found", id);
                return Ok(None);
            }
            log::error!("{:?}", res);
            return Err(From::from("Error code not ok"));
        }

        let activity = res.json::<DetailedActivity>().await?;
        Ok(Some(activity))
    }

    pub async fn get_logged_in_athlete_activities(
        &self,
        before: i32,
        after: i32,
        page: i32,
        per_page: i32,
        access_token: &String,
    ) -> Result<Vec<DetailedActivity>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        debug!("get_logged_in_athlete_activities");
        let url = format!(
            "{}/athlete/activities?before={}&after={}&page={}&per_page={}",
            self.configuration.base_path, before, after, page, per_page
        );
        let authorization = format!("Bearer {}", access_token);
        let res = Client::new()
            .get(url.as_str())
            .header("Authorization", authorization)
            .send()
            .await?;

        if res.status().clone() != StatusCode::OK {
            log::error!("{:?}", res);
            return Err(From::from("Error code not ok"));
        }

        let activities = res.json::<Vec<DetailedActivity>>().await?;
        Ok(activities)
    }
}
