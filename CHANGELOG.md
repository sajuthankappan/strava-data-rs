## Unreleased

* Breaking: `get_logged_in_athlete_activities` now returns `Vec<SummaryActivity>` instead of `Vec<DetailedActivity>`, matching what Strava returns from `/athlete/activities`. `SummaryActivity` drops `description`, `calories`, `device_name` and `embed_token`, which were always `None` there, and adds `hide_from_home`
* Make `weighted_average_watts` public on `DetailedActivity`

## 0.8.1

* Add `sport_type` to `DetailedActivity`, as a new `SportType` enum. Sport types unknown to this crate are kept as `SportType::Other` with the exact value Strava sent
* Add a `rustls` feature to use rustls instead of native TLS
* Upgrade reqwest from 0.11 to 0.13 (native TLS remains the default)
* Replace unmaintained `dotenv` dev-dependency with `dotenvy`
* Fix integration tests panicking when initializing the logger more than once

## 0.8.0

* Breaking: `before` and `after` in `get_logged_in_athlete_activities` are now `Option<i64>`, and are omitted from the request when `None`
* Breaking: Return a typed `Error` from API calls instead of `Box<dyn Error>`, distinguishing unauthorized, rate-limited, other status, request and decode failures
* Breaking: Remove the `wasm` feature, which had not compiled since 0.6.0
* Update to Rust 2024 edition, with a minimum supported Rust version of 1.87
* Reuse a single HTTP client across requests instead of creating one per request
* Take `access_token` as `&str` instead of `&String`
* Replace Rc configuration with Arc configuration
* Add `as_str`, `Display` and `FromStr` to `ActivityType`
* Make `LatLng` deserialize from `[lat, lng]` arrays and enable `start_latlng` / `end_latlng` on `DetailedActivity`

## 0.6.4 & 0.6.5

* Fix visibilty attributes of new webhook models

## 0.6.3

* Add additional models for webhooks processing

## 0.6.2

* Add hub.mode param for for webhooks verification params

## 0.6.1

* Add models for webhooks processing

## 0.6.0

* Remove getset dependency

## 0.5.0

* Upgrade reqwest to 0.11.4

## 0.4.1

* Update get_activity_by_id to return option instead of just result

## 0.3.2

* Correct dev-dependencies to remove async-std from dependencies

## 0.3.1

* Change access_token as ref

## 0.3.0

Released 2020-04-20.

### Added

* Added get_logged_in_athlete_activities

## 0.2.3

* Fixed: wasm fixes for new models

## 0.2.2

* Added SummaryAthlete model

## 0.2.1

* Fixed: Export DetailedAthlete model and added additional missed models

## 0.2.0

* Added DetailedAthlete model

## 0.1.0

* First preview release
