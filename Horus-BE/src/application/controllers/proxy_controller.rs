use actix_web::{
    http::header::{HeaderMap as ActixHeaderMap, HeaderName, HeaderValue as ActixHeaderValue},
    http::Method as ActixMethod,
    http::StatusCode,
    web, HttpRequest, HttpResponse,
};
use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Method,
};
// use std::collections::HashMap;
use std::str::FromStr;
use tracing::{debug, error, info};

// If you rely on Octocrab, you can still get its underlying reqwest client
// or build your own client with the same token.

#[allow(clippy::async_yields_async)]
pub async fn catch_all(
    req: HttpRequest,
    body: Bytes,             // Full request body
    path: web::Path<String>, // the {tail:.*} capture
) -> HttpResponse {
    let tail = path.into_inner();
    let query_string = req.query_string();

    info!(
        method = ?req.method(),
        path = ?tail,
        query = ?query_string,
        "Received proxy request"
    );

    // Combine your base GitHub URL + tail
    let github_url = if query_string.is_empty() {
        format!("https://api.github.com/{}", tail)
    } else {
        format!("https://api.github.com/{}?{}", tail, query_string)
    };

    debug!(target_url = ?github_url, "Forwarding request to GitHub");

    // Which HTTP method?
    let method = match *req.method() {
        ActixMethod::GET => Method::GET,
        ActixMethod::POST => Method::POST,
        ActixMethod::PUT => Method::PUT,
        ActixMethod::DELETE => Method::DELETE,
        ActixMethod::HEAD => Method::HEAD,
        ActixMethod::OPTIONS => Method::OPTIONS,
        ActixMethod::PATCH => Method::PATCH,
        _ => Method::GET,
    };

    // Get GitHub token
    let token = match std::env::var("GITHUB_TOKEN") {
        Ok(t) if !t.is_empty() => {
            debug!("Using GitHub token for authentication");
            Some(t)
        }
        _ => {
            debug!("No GitHub token found, proceeding without authentication");
            // Raise an error
            return HttpResponse::BadGateway().json(serde_json::json!({
                "error": "No GitHub token found",
                "details": "Please set the GITHUB_TOKEN environment variable"
            }));
        }
    };

    // Create or use a reusable reqwest client
    let client = Client::new();
    let mut request_builder = client.request(method, &github_url);

    // Add auth if token exists
    if let Some(token) = token {
        request_builder = request_builder.bearer_auth(token);
    }

    // Forward headers
    let mut headers = HeaderMap::new();
    copy_some_headers(req.headers(), &mut headers);
    request_builder = request_builder.headers(headers);

    // Add body for POST/PUT/PATCH
    request_builder = request_builder.body(body);

    // Send to GitHub
    let res = match request_builder.send().await {
        Ok(r) => r,
        Err(e) => {
            error!(
                error = ?e,
                url = ?github_url,
                "Failed to forward request to GitHub"
            );
            return HttpResponse::BadGateway().json(serde_json::json!({
                "error": "Failed to forward request",
                "details": e.to_string()
            }));
        }
    };

    build_http_response(res).await
}

// Utility to copy selected headers
fn copy_some_headers(original: &ActixHeaderMap, target: &mut HeaderMap) {
    let headers_to_forward = ["user-agent", "accept", "content-type"];

    for header in headers_to_forward.iter() {
        if let Some(v) = original.get(*header) {
            if let Ok(val) = v.to_str() {
                if let Ok(header_val) = HeaderValue::from_str(val) {
                    target.insert(*header, header_val);
                }
            }
        }
    }
}

// Convert reqwest::Response to actix_web::HttpResponse
async fn build_http_response(resp: reqwest::Response) -> HttpResponse {
    let status = resp.status();
    let status_code = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);

    info!(
        status = ?status,
        "Received response from GitHub"
    );

    // Forward response headers
    let mut builder = HttpResponse::build(status_code);
    for (key, value) in resp.headers() {
        if let Ok(v) = value.to_str() {
            if let Ok(name) = HeaderName::from_str(key.as_str()) {
                if let Ok(val) = ActixHeaderValue::from_str(v) {
                    builder.insert_header((name, val));
                }
            }
        }
    }

    // Get response body
    match resp.bytes().await {
        Ok(bytes) => builder.body(bytes),
        Err(e) => {
            error!(error = ?e, "Failed to read GitHub response body");
            HttpResponse::BadGateway().json(serde_json::json!({
                "error": "Failed to read GitHub response",
                "details": e.to_string()
            }))
        }
    }
}
