# strava-data-rs

Rust client for the Strava API, published to crates.io as `strava-data`.

## Git workflow

- This repo has a single maintainer, so commit and push directly to `master`. Don't create feature branches or PRs.
- Only commit or push when explicitly asked.
- Don't publish to crates.io. The maintainer publishes.

## Build and test

- `cargo test --lib` runs the offline tests: unit tests in `#[cfg(test)]` modules, plus `activities_api` tests against a local `wiremock` server.
- CI (`.github/workflows/ci.yml`) runs fmt, clippy with `-D warnings`, `cargo test --lib`, and `cargo check` on the MSRV (1.87), for both the default (native-tls) and `rustls` features. Keep the MSRV job in sync with `rust-version`.
- `tests/integration_tests.rs` calls the live Strava API and needs a `.env` with `ACCESS_TOKEN`, `ACTIVITY_ID`, `BEFORE` and `AFTER`. Without it these tests fail, which is expected.

## Wire-format stability

A downstream app stores values from these models in Postgres, so serialized output must not change:

- Never rename `ActivityType` variants or change their serde representation.
- `ActivityType::as_str()` must stay byte-identical to the serde value. A test enforces this, and new variants must be added to its list.
- Adding fields to a `Serialize` model changes its serialized output. Call that out when you do it.
- `SportType` is open-ended: Strava keeps adding values, so unknown ones deserialize to `SportType::Other` rather than failing. Keep that fallback.

## Conventions

- One model per file in `src/models/`, re-exported from `src/models.rs`.
- Model fields are `pub Option<T>` with an explicit `#[serde(rename = "...")]` and a `///` doc comment taken from the Strava API docs.
- Code is formatted with `cargo fmt` (default settings), and CI enforces it with `cargo fmt --check`.
- Add a line to the `## Unreleased` section of `CHANGELOG.md` for user-visible changes.
- Versioning: breaking changes bump the minor version (`0.8.x` → `0.9.0`), additive changes bump the patch version.
