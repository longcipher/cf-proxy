use regex::Regex;
use worker::*;

use crate::config::ProxyConfig;

/// Apply request middleware
pub fn apply_request_middleware(req: Request, config: &ProxyConfig) -> Result<Request> {
    // Access control check
    if !check_access_control(&req, config)? {
        return Err(Error::from("Access denied"));
    }

    // Simply return the request without modifying headers
    // Headers will be modified in create_proxy_request instead
    Ok(req)
}

/// Apply response middleware
pub fn apply_response_middleware(response: Response, _config: &ProxyConfig) -> Result<Response> {
    let headers = response.headers().clone();

    // Add security headers
    headers.set("X-Content-Type-Options", "nosniff")?;
    headers.set("X-Frame-Options", "DENY")?;
    headers.set("X-XSS-Protection", "1; mode=block")?;
    headers.set("Referrer-Policy", "strict-origin-when-cross-origin")?;

    // Add proxy identification
    headers.set("X-Proxied-By", "Cloudflare-Workers")?;

    // Remove sensitive headers
    headers.delete("Server")?;
    headers.delete("X-Powered-By")?;

    // Simplified response construction
    Ok(response)
}

/// Check access control
fn check_access_control(req: &Request, config: &ProxyConfig) -> Result<bool> {
    let cf = req.cf();

    for rule in &config.access_rules {
        match rule.rule_type.as_str() {
            "deny_ip" => {
                if let Ok(Some(ip)) = req.headers().get("CF-Connecting-IP") {
                    if ip == rule.pattern {
                        console_log!("Access denied for IP: {}", ip);
                        return Ok(false);
                    }
                }
            }
            "allow_country" => {
                if let Some(cf_data) = cf {
                    if let Some(country) = cf_data.country() {
                        if country.as_str() != rule.pattern {
                            console_log!("Access denied for country: {}", country.as_str());
                            return Ok(false);
                        }
                    }
                }
            }
            "deny_country" => {
                if let Some(cf_data) = cf {
                    if let Some(country) = cf_data.country() {
                        if country.as_str() == rule.pattern {
                            console_log!("Access denied for country: {}", country.as_str());
                            return Ok(false);
                        }
                    }
                }
            }
            "deny_user_agent" => {
                if let Ok(Some(user_agent)) = req.headers().get("User-Agent") {
                    if let Ok(regex) = Regex::new(&rule.pattern) {
                        if regex.is_match(&user_agent) {
                            console_log!("Access denied for User-Agent: {}", user_agent);
                            return Ok(false);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(true)
}

/// Add CORS headers
#[allow(dead_code)]
pub fn add_cors_headers(response: &mut Response) -> Result<()> {
    let headers = response.headers().clone();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, DELETE, OPTIONS",
    )?;
    headers.set(
        "Access-Control-Allow-Headers",
        "Content-Type, Authorization, X-Requested-With",
    )?;
    headers.set("Access-Control-Max-Age", "86400")?;
    Ok(())
}

/// Handle OPTIONS preflight request
#[allow(dead_code)]
pub fn handle_options_request() -> Result<Response> {
    let mut response = Response::empty()?;
    add_cors_headers(&mut response)?;
    Ok(response)
}
