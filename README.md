# strava-data &emsp; [![Latest Version]][crates.io] [![Docs]][docs.rs] [![CI Status]][actions]
[Latest Version]: https://img.shields.io/crates/v/strava-data.svg
[crates.io]: https://crates.io/crates/strava-data
[Docs]: https://docs.rs/strava-data/badge.svg
[docs.rs]: https://docs.rs/strava-data
[CI Status]: https://github.com/sajuthankappan/strava-data-rs/actions/workflows/ci.yml/badge.svg?branch=master
[actions]: https://github.com/sajuthankappan/strava-data-rs/actions/workflows/ci.yml

**Strava data API client for Rust**

## Features

- Get an activity by id
- List the logged-in athlete's activities
- Models to help processing incoming webhooks

## TLS

By default, HTTPS uses the platform's native TLS (OpenSSL on Linux). To use rustls instead:

```toml
strava-data = { version = "0.9", default-features = false, features = ["rustls"] }
```

## Access tokens

This crate needs an OAuth access token for the athlete. To obtain and refresh tokens, see [`strava-auth`](https://crates.io/crates/strava-auth).

## Usage example

Get an activity

```rust
use strava_data::{ApiClient, Configuration};

let access_token = "<access_token>";
let activity_id = 1234567890;
let client = ApiClient::new(Configuration::new());
let response = client
    .activities_api
    .get_activity_by_id(activity_id, access_token)
    .await?;

// None if the activity does not exist
if let Some(activity) = response.data {
    println!("{:?}: {:?}", activity.name, activity.activity_type);
}
```

List the logged-in athlete's activities, optionally between two Unix timestamps. These are returned as `SummaryActivity`, so use `get_activity_by_id` for fields such as `description` and `calories`

```rust
let before = Some(1767225600);
let after = Some(1735689600);
let page = 1;
let per_page = 30;
let activities = client
    .activities_api
    .get_logged_in_athlete_activities(before, after, page, per_page, access_token)
    .await?
    .data;
```

## Rate limits

Every successful call returns an `ApiResponse`, which holds the response `data` and the `rate_limit` Strava reported in its headers. `rate_limit` has an `overall` window (`X-RateLimit-*`) and a `read` window (`X-ReadRateLimit-*`), each with 15-minute and daily limits and usage. Any of these is `None` if Strava did not send the headers.

```rust
let response = client
    .activities_api
    .get_activity_by_id(activity_id, access_token)
    .await?;

if let Some(read) = response.rate_limit.and_then(|rate_limit| rate_limit.read) {
    println!(
        "{}/{} read requests in the last 15 minutes, {}/{} today",
        read.short_term_usage, read.short_term_limit, read.daily_usage, read.daily_limit
    );
}
```

## Error handling

API calls return `strava_data::Error`, which distinguishes the failures callers usually handle differently.

```rust
use strava_data::{ApiResponse, Error};

match client.activities_api.get_activity_by_id(activity_id, access_token).await {
    Ok(ApiResponse { data: Some(activity), .. }) => println!("found {:?}", activity.name),
    Ok(ApiResponse { data: None, .. }) => println!("activity not found"),
    // Access token missing, invalid, expired or lacking the required scope
    Err(Error::Unauthorized { body }) => eprintln!("refresh the access token: {body}"),
    // Strava rate limit exceeded; `rate_limit` shows which window was exceeded
    Err(Error::RateLimited { body, rate_limit }) => eprintln!("rate limited: {body} {rate_limit:?}"),
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
