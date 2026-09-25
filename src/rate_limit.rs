use reqwest::header::HeaderMap;

/// A successful API response, with the rate limit reported alongside it
#[derive(Debug, Clone, PartialEq)]
pub struct ApiResponse<T> {
    /// The response data
    pub data: T,
    /// The rate limit reported by Strava, or `None` if no rate limit headers were sent
    pub rate_limit: Option<RateLimit>,
}

/// The rate limits reported in Strava's response headers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimit {
    /// The overall limit, from `X-RateLimit-Limit` and `X-RateLimit-Usage`
    pub overall: Option<RateLimitWindow>,
    /// The limit for read requests, from `X-ReadRateLimit-Limit` and `X-ReadRateLimit-Usage`
    pub read: Option<RateLimitWindow>,
}

/// Limits and usage for the 15-minute and daily windows
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimitWindow {
    /// The number of requests allowed every 15 minutes
    pub short_term_limit: u32,
    /// The number of requests allowed every day
    pub daily_limit: u32,
    /// The number of requests made in the current 15-minute window
    pub short_term_usage: u32,
    /// The number of requests made today
    pub daily_usage: u32,
}

impl RateLimit {
    /// Parse the rate limit headers, returning `None` if neither pair is present and valid
    pub(crate) fn from_headers(headers: &HeaderMap) -> Option<RateLimit> {
        let overall =
            RateLimitWindow::from_headers(headers, "x-ratelimit-limit", "x-ratelimit-usage");
        let read = RateLimitWindow::from_headers(
            headers,
            "x-readratelimit-limit",
            "x-readratelimit-usage",
        );
        if overall.is_none() && read.is_none() {
            return None;
        }
        Some(RateLimit { overall, read })
    }
}

impl RateLimitWindow {
    fn from_headers(headers: &HeaderMap, limit: &str, usage: &str) -> Option<RateLimitWindow> {
        let (short_term_limit, daily_limit) = parse_pair(headers, limit)?;
        let (short_term_usage, daily_usage) = parse_pair(headers, usage)?;
        Some(RateLimitWindow {
            short_term_limit,
            daily_limit,
            short_term_usage,
            daily_usage,
        })
    }
}

/// Parse a `"<15-minute>,<daily>"` header value
fn parse_pair(headers: &HeaderMap, name: &str) -> Option<(u32, u32)> {
    let value = headers.get(name)?.to_str().ok()?;
    let (short_term, daily) = value.split_once(',')?;
    Some((short_term.trim().parse().ok()?, daily.trim().parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    fn headers(pairs: &[(&'static str, &'static str)]) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for (name, value) in pairs {
            headers.insert(*name, HeaderValue::from_static(value));
        }
        headers
    }

    #[test]
    fn parses_overall_and_read_windows() {
        let rate_limit = RateLimit::from_headers(&headers(&[
            ("X-RateLimit-Limit", "200,2000"),
            ("X-RateLimit-Usage", "20,300"),
            ("X-ReadRateLimit-Limit", "100,1000"),
            ("X-ReadRateLimit-Usage", "10,150"),
        ]))
        .unwrap();
        assert_eq!(
            rate_limit.overall,
            Some(RateLimitWindow {
                short_term_limit: 200,
                daily_limit: 2000,
                short_term_usage: 20,
                daily_usage: 300,
            })
        );
        assert_eq!(
            rate_limit.read,
            Some(RateLimitWindow {
                short_term_limit: 100,
                daily_limit: 1000,
                short_term_usage: 10,
                daily_usage: 150,
            })
        );
    }

    #[test]
    fn missing_pair_leaves_window_empty() {
        let rate_limit = RateLimit::from_headers(&headers(&[
            ("X-RateLimit-Limit", "200,2000"),
            ("X-RateLimit-Usage", "20,300"),
            ("X-ReadRateLimit-Limit", "100,1000"),
        ]))
        .unwrap();
        assert!(rate_limit.overall.is_some());
        assert!(rate_limit.read.is_none());
    }

    #[test]
    fn malformed_values_are_ignored() {
        for value in ["200", "200,abc", "", "200,2000,3"] {
            let rate_limit = RateLimit::from_headers(&headers(&[
                ("X-RateLimit-Limit", value),
                ("X-RateLimit-Usage", "20,300"),
            ]));
            assert!(rate_limit.is_none(), "{value:?} should not parse");
        }
    }

    #[test]
    fn no_headers_is_none() {
        assert!(RateLimit::from_headers(&HeaderMap::new()).is_none());
    }
}
