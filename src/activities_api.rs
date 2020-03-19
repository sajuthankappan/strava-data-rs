use log::info;
use std::rc::Rc;
use surf;

use crate::models::DetailedActivity;
use crate::configuration::Configuration;

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
        id: u64,
        access_token: String,
    ) -> Result<DetailedActivity, Box<dyn std::error::Error + Send + Sync + 'static>> {
        info!("get_activity_by_id {}", id);
        let uri = format!("{}/activities/{id}", self.configuration.base_path, id=id);
        let authorization = format!("Bearer {}", access_token);
        let activity: DetailedActivity = surf::get(uri).set_header("Authorization", authorization).recv_json().await?;
        Ok(activity)
    }
}
