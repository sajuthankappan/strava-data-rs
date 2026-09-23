# strava-data &emsp; [![Latest Version]][crates.io] [![Docs]][docs.rs]
[Latest Version]: https://img.shields.io/crates/v/strava-data.svg
[crates.io]: https://crates.io/crates/strava-data
[Docs]: https://docs.rs/strava-data/badge.svg
[docs.rs]: https://docs.rs/strava-data

**Strava data API client for Rust**

## Features

- Get an activity by id
- List the logged-in athlete's activities
- Models to help processing incoming webhooks

This crate needs an OAuth access token for the athlete. To obtain and refresh tokens, see [`strava-auth`](https://crates.io/crates/strava-auth).

## Usage example

Get an activity

```rust
use strava_data::{ApiClient, Configuration};

let access_token = "<access_token>";
let activity_id = 1234567890;
let client = ApiClient::new(Configuration::new());
let activity = client
    .activities_api
    .get_activity_by_id(activity_id, access_token)
    .await?;

// None if the activity does not exist
if let Some(activity) = activity {
    println!("{:?}: {:?}", activity.name, activity.activity_type);
}
```

List the logged-in athlete's activities between two Unix timestamps

```rust
let before = 1767225600;
let after = 1735689600;
let page = 1;
let per_page = 30;
let activities = client
    .activities_api
    .get_logged_in_athlete_activities(before, after, page, per_page, access_token)
    .await?;
```

## Error handling

API calls return `strava_data::Error`, which distinguishes the failures callers usually handle differently.

```rust
use strava_data::Error;

match client.activities_api.get_activity_by_id(activity_id, access_token).await {
    Ok(Some(activity)) => println!("found {:?}", activity.name),
    Ok(None) => println!("activity not found"),
    // Access token missing, invalid, expired or lacking the required scope
    Err(Error::Unauthorized { body }) => eprintln!("refresh the access token: {body}"),
    // Strava rate limit exceeded; retry later
    Err(Error::RateLimited { body }) => eprintln!("rate limited: {body}"),
    // Any other error response from Strava
    Err(Error::Status { status, body }) => eprintln!("HTTP {status}: {body}"),
    // Network / TLS errors, or a response that does not match the models
    Err(e) => eprintln!("request failed: {e}"),
}
```

For more details, please see the [tests] folder

[tests]: https://github.com/sajuthankappan/strava-data-rs/tree/master/tests

## Maintainer

Maintained by [Saju Thankappan](https://github.com/sajuthankappan), creator of [Smito One](https://smito.in), which uses this crate for its Strava integration.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
