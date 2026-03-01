use url::Url;

pub const VERCEL_ORIGIN: &str = "https://vercel.com";

pub fn origin_or_vercel(raw_url: &str) -> String {
    Url::parse(raw_url)
        .map(|parsed| parsed.origin().ascii_serialization())
        .unwrap_or_else(|_| VERCEL_ORIGIN.to_string())
}

pub fn origin_with_path_or_vercel(raw_url: &str, path: &str) -> String {
    let origin = origin_or_vercel(raw_url);
    let trimmed_path = path.trim_start_matches('/');
    if trimmed_path.is_empty() {
        origin
    } else {
        format!("{origin}/{trimmed_path}")
    }
}

#[cfg(test)]
mod tests {
    use super::{origin_or_vercel, origin_with_path_or_vercel};

    #[test]
    fn test_origin_or_vercel() {
        assert_eq!(
            origin_or_vercel("https://example.com/path?x=1#y"),
            "https://example.com"
        );
        assert_eq!(origin_or_vercel("not-a-url"), "https://vercel.com");
    }

    #[test]
    fn test_origin_with_path_or_vercel() {
        assert_eq!(
            origin_with_path_or_vercel("https://example.com/foo", "notifications/cli-login/turbo/"),
            "https://example.com/notifications/cli-login/turbo/"
        );
        assert_eq!(
            origin_with_path_or_vercel(
                "https://example.com/foo",
                "/notifications/cli-login/turbo/"
            ),
            "https://example.com/notifications/cli-login/turbo/"
        );
        assert_eq!(
            origin_with_path_or_vercel("not-a-url", "account/billing"),
            "https://vercel.com/account/billing"
        );
    }
}
