use log::debug;
use std::rc::Rc;
use surf;

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
    ) -> Result<DetailedActivity, Box<dyn std::error::Error + Send + Sync + 'static>> {
        debug!("get_activity_by_id {}", id);
        let uri = format!("{}/activities/{id}", self.configuration.base_path, id = id);
        let authorization = format!("Bearer {}", access_token);
        surf::get(uri)
            .set_header("Authorization", authorization)
            .recv_json::<DetailedActivity>()
            .await
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
        let uri = format!(
            "{}/athlete/activities?before={}&after={}&page={}&per_page={}",
            self.configuration.base_path, before, after, page, per_page
        );
        let authorization = format!("Bearer {}", access_token);
        surf::get(uri)
            .set_header("Authorization", authorization)
            .recv_json::<Vec<DetailedActivity>>()
            .await
    }
}
