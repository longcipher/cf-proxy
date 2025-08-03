use worker::*;

/// Utility functions module
/// Parse URL path
#[allow(dead_code)]
pub fn parse_url_path(url: &str) -> Result<String> {
    let url_obj = url::Url::parse(url).map_err(|_| Error::from("Invalid URL"))?;
    Ok(url_obj.path().to_string())
}

/// Parse query parameters
#[allow(dead_code)]
pub fn parse_query_string(url: &str) -> Result<String> {
    let url_obj = url::Url::parse(url).map_err(|_| Error::from("Invalid URL"))?;
    Ok(url_obj.query().unwrap_or("").to_string())
}

/// Format response time
#[allow(dead_code)]
pub fn format_duration_ms(ms: f64) -> String {
    if ms < 1000.0 {
        format!("{ms:.2}ms")
    } else {
        format!("{:.2}s", ms / 1000.0)
    }
}

/// Validate URL format
#[allow(dead_code)]
pub fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

/// Generate request ID
#[allow(dead_code)]
pub fn generate_request_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Safely get header value
#[allow(dead_code)]
pub fn get_header_value(headers: &Headers, name: &str) -> Option<String> {
    headers.get(name).ok().flatten()
}

/// Get client IP
#[allow(dead_code)]
pub fn get_client_ip(headers: &Headers, cf: Option<&Cf>) -> Option<String> {
    // Prefer using Cloudflare provided IP
    if let Some(_cf_data) = cf {
        // Note: In actual Cloudflare Workers, might need different API
        // Here use headers as fallback
    }

    if let Ok(Some(cf_ip)) = headers.get("CF-Connecting-IP") {
        return Some(cf_ip);
    }

    if let Ok(Some(x_forwarded_for)) = headers.get("X-Forwarded-For") {
        if let Some(first_ip) = x_forwarded_for.split(',').next() {
            return Some(first_ip.trim().to_string());
        }
    }

    if let Ok(Some(x_real_ip)) = headers.get("X-Real-IP") {
        return Some(x_real_ip);
    }

    None
}

/// Parse user agent information
#[allow(dead_code)]
pub fn parse_user_agent(user_agent: &str) -> UserAgentInfo {
    UserAgentInfo {
        browser: extract_browser(user_agent),
        os: extract_os(user_agent),
        is_mobile: is_mobile_device(user_agent),
        is_bot: is_bot(user_agent),
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UserAgentInfo {
    pub browser: String,
    pub os: String,
    pub is_mobile: bool,
    pub is_bot: bool,
}

#[allow(dead_code)]
fn extract_browser(ua: &str) -> String {
    if ua.contains("Chrome") {
        "Chrome".to_string()
    } else if ua.contains("Firefox") {
        "Firefox".to_string()
    } else if ua.contains("Safari") {
        "Safari".to_string()
    } else if ua.contains("Edge") {
        "Edge".to_string()
    } else {
        "Unknown".to_string()
    }
}

#[allow(dead_code)]
fn extract_os(ua: &str) -> String {
    if ua.contains("Windows") {
        "Windows".to_string()
    } else if ua.contains("Mac OS") {
        "macOS".to_string()
    } else if ua.contains("Linux") {
        "Linux".to_string()
    } else if ua.contains("Android") {
        "Android".to_string()
    } else if ua.contains("iOS") {
        "iOS".to_string()
    } else {
        "Unknown".to_string()
    }
}

#[allow(dead_code)]
fn is_mobile_device(ua: &str) -> bool {
    ua.contains("Mobile") || ua.contains("Android") || ua.contains("iPhone") || ua.contains("iPad")
}

#[allow(dead_code)]
fn is_bot(ua: &str) -> bool {
    let bot_indicators = [
        "bot",
        "crawler",
        "spider",
        "scraper",
        "crawl",
        "scanner",
        "Googlebot",
        "Bingbot",
        "facebookexternalhit",
        "Twitterbot",
    ];

    let ua_lower = ua.to_lowercase();
    bot_indicators
        .iter()
        .any(|&indicator| ua_lower.contains(indicator))
}

/// Base64 encoding
#[allow(dead_code)]
pub fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

/// Base64 decoding
#[allow(dead_code)]
pub fn base64_decode(data: &str) -> Result<Vec<u8>> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|_| Error::from("Invalid base64"))
}

/// Calculate SHA-256 hash
#[allow(dead_code)]
pub fn sha256_hash(data: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

/// Verify HMAC-SHA256 signature
#[allow(dead_code)]
pub fn verify_hmac_sha256(data: &str, signature: &str, secret: &str) -> bool {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(format!("{secret}{data}").as_bytes());
    let computed_hash = hex::encode(hasher.finalize());
    computed_hash == signature
}

/// Clean and validate path
#[allow(dead_code)]
pub fn clean_path(path: &str) -> String {
    // Remove extra slashes and relative path components
    let parts: Vec<&str> = path.split('/').collect();
    let mut cleaned: Vec<&str> = Vec::new();

    for part in parts.iter() {
        match *part {
            "" | "." => continue,
            ".." => {
                cleaned.pop();
            }
            _ => cleaned.push(part),
        }
    }

    let result = format!("/{}", cleaned.join("/"));
    if result == "/" {
        result
    } else {
        result.trim_end_matches('/').to_string()
    }
}
