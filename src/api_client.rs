use super::activities_api::ActivitiesApi;
use super::configuration::Configuration;
use std::rc::Rc;

pub struct ApiClient {
    pub configuration: Rc<Configuration>,
    pub activities_api: Box<ActivitiesApi>,
}

impl ApiClient {
    pub fn new(configuration: Configuration) -> ApiClient {
        let rc = Rc::new(configuration);

        ApiClient {
            configuration: rc.clone(),
            activities_api: Box::new(ActivitiesApi::new(rc.clone())),
        }
    }
}
