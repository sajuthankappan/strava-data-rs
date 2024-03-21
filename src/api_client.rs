use std::sync::Arc;

use super::activities_api::ActivitiesApi;
use super::configuration::Configuration;

pub struct ApiClient {
    pub configuration: Arc<Configuration>,
    pub activities_api: Box<ActivitiesApi>,
}

impl ApiClient {
    pub fn new(configuration: Configuration) -> ApiClient {
        let rc = Arc::new(configuration);

        ApiClient {
            configuration: rc.clone(),
            activities_api: Box::new(ActivitiesApi::new(rc.clone())),
        }
    }
}
