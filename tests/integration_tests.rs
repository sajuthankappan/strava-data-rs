extern crate strava_data;

use async_std::task;
use std::env;
use strava_data::{ApiClient, Configuration};

#[test]
fn test_get_activity_by_id() {
    env_logger::init();

    let activity_id = env::var("ACTIVITY_ID").unwrap().parse().unwrap();
    let access_token = env::var("ACCESS_TOKEN").unwrap();

    let strava_api_configuration = Configuration::new();
    let strava_api_client = ApiClient::new(strava_api_configuration);
    let strava_activity = task::block_on(
        strava_api_client
            .activities_api
            .get_activity_by_id(activity_id, &access_token),
    )
    .unwrap();
    dbg!(strava_activity);
}

#[test]
fn test_get_logged_in_athlete_activities() {
    env_logger::init();

    let before = env::var("BEFORE").unwrap().parse().unwrap();
    let after = env::var("AFTER").unwrap().parse().unwrap();
    let page = 1;
    let per_page = 100;
    let access_token = env::var("ACCESS_TOKEN").unwrap();

    let strava_api_configuration = Configuration::new();
    let strava_api_client = ApiClient::new(strava_api_configuration);
    let strava_activities = task::block_on(
        strava_api_client
            .activities_api
            .get_logged_in_athlete_activities(before, after, page, per_page, &access_token),
    )
    .unwrap();
    dbg!(strava_activities);
}
