# strava-data-rs

Rust client for the Strava API, published to crates.io as `strava-data`.

## Git workflow

- This repo has a single maintainer, so commit and push directly to `master`. Don't create feature branches or PRs.
- Only commit or push when explicitly asked.
- Don't publish to crates.io. The maintainer publishes.

## Build and test

- `cargo test` runs the unit tests, which live in `#[cfg(test)]` modules next to the models.
- `tests/integration_tests.rs` calls the live Strava API and needs a `.env` with `ACCESS_TOKEN`, `ACTIVITY_ID`, `BEFORE` and `AFTER`. Without it these tests fail, which is expected.

## Wire-format stability

A downstream app stores values from these models in Postgres, so serialized output must not change:

- Never rename `ActivityType` variants or change their serde representation.
- `ActivityType::as_str()` must stay byte-identical to the serde value. A test enforces this, and new variants must be added to its list.
- Adding fields to a `Serialize` model changes its serialized output. Call that out when you do it.

## Conventions

- One model per file in `src/models/`, re-exported from `src/models.rs`.
- Model fields are `pub Option<T>` with an explicit `#[serde(rename = "...")]` and a `///` doc comment taken from the Strava API docs.
- Match each file's existing indentation: most model files use 2 spaces, `activity_type.rs` uses 4.
- Add a line to the `## Unreleased` section of `CHANGELOG.md` for user-visible changes.
- Additive changes bump the pre-release version (e.g. `0.7.0-alpha.1` → `0.7.0-alpha.2`).
