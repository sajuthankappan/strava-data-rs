extern crate strava_data;

use std::env;
use strava_data::{ApiClient, Configuration};

#[tokio::test]
async fn test_get_activity_by_id() {
    dotenv::dotenv().ok();
    env_logger::init();

    let activity_id = env::var("ACTIVITY_ID").unwrap().parse().unwrap();
    let access_token = env::var("ACCESS_TOKEN").unwrap();

    let strava_api_configuration = Configuration::new();
    let strava_api_client = ApiClient::new(strava_api_configuration);
    let strava_activity = strava_api_client
        .activities_api
        .get_activity_by_id(activity_id, &access_token)
        .await
        .unwrap();
    dbg!(strava_activity);
}

#[tokio::test]
async fn test_get_logged_in_athlete_activities() {
    dotenv::dotenv().ok();
    env_logger::init();

    let before = env::var("BEFORE").unwrap().parse().unwrap();
    let after = env::var("AFTER").unwrap().parse().unwrap();
    let page = 1;
    let per_page = 100;
    let access_token = env::var("ACCESS_TOKEN").unwrap();

    let strava_api_configuration = Configuration::new();
    let strava_api_client = ApiClient::new(strava_api_configuration);
    let strava_activities = strava_api_client
        .activities_api
        .get_logged_in_athlete_activities(before, after, page, per_page, &access_token)
        .await
        .unwrap();
    dbg!(strava_activities);
}
