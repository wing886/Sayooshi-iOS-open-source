//! Desktop Siglus Twitter integration.
//!
//! The original engine implements this in `eng_twitter.cpp` with WinINet,
//! OAuth 1.0a and the Twitter v1.1 upload/status APIs.  Keep the protocol and
//! request construction compatible here, but use the project's existing
//! `ureq` transport and a cross-platform sidecar instead of the Windows
//! registry.

#![cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, anyhow, bail};

use crate::original_save;
use crate::runtime::CommandContext;

const TWITTER_API_DOMAIN: &str = "api.twitter.com";
const TWITTER_UPLOAD_DOMAIN: &str = "upload.twitter.com";
const TWITTER_USER_AGENT: &str = "SiglusEngine/1.0.0";
const TWITTER_SIGNATURE_METHOD: &str = "HMAC-SHA1";
const TWITTER_OAUTH_VERSION: &str = "1.0";
const TWITTER_STATE_FILE: &str = "twitter_state.dat";

static NONCE_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Default)]
pub struct TwitterState {
    pub request_token: String,
    pub request_token_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
    pub user_id: String,
    pub user_name: String,
    pub screen_name: String,
    pub loaded: bool,
}

impl TwitterState {
    pub fn is_authorized(&self) -> bool {
        !self.access_token.is_empty()
            && !self.access_token_secret.is_empty()
            && !self.user_id.is_empty()
            && !self.user_name.is_empty()
            && !self.screen_name.is_empty()
    }

    pub fn clear_authorization(&mut self) {
        self.request_token.clear();
        self.request_token_secret.clear();
        self.access_token.clear();
        self.access_token_secret.clear();
        self.user_id.clear();
        self.user_name.clear();
        self.screen_name.clear();
    }
}

#[derive(Debug, Clone)]
pub struct TwitterDialogRequest {
    pub image_path: PathBuf,
    pub image_rgba: Vec<u8>,
    pub image_width: u32,
    pub image_height: u32,
    pub initial_text: String,
}

fn gameexe_value(ctx: &CommandContext, key: &str) -> String {
    ctx.tables
        .gameexe
        .as_ref()
        .and_then(|cfg| cfg.get_unquoted(key))
        .unwrap_or("")
        .to_string()
}

fn state_path(project_dir: &Path) -> PathBuf {
    original_save::save_dir(project_dir).join(TWITTER_STATE_FILE)
}

fn percent_encode(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut result = String::with_capacity(value.len());
    for c in value.as_bytes().iter().copied() {
        if c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_' | b'.' | b'~') {
            result.push(c as char);
        } else {
            result.push('%');
            result.push(HEX[(c >> 4) as usize] as char);
            result.push(HEX[(c & 0x0f) as usize] as char);
        }
    }
    result
}

fn percent_decode(value: &str) -> String {
    fn hex(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }

    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let (Some(hi), Some(lo)) = (hex(bytes[i + 1]), hex(bytes[i + 2]))
        {
            out.push((hi << 4) | lo);
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn query_to_pairs(query: &str) -> Vec<(String, String)> {
    // Match tona3 query_str_to_map(): OAuth endpoint/callback values are split
    // on '&' and '=', but are not URL-decoded at this stage.
    query
        .split('&')
        .filter_map(|entry| {
            let mut parts = entry.split('=');
            let key = parts.next()?;
            let value = parts.next()?;
            if parts.next().is_some() {
                return None;
            }
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

fn query_value<'a>(pairs: &'a [(String, String)], key: &str) -> Option<&'a str> {
    pairs
        .iter()
        .find(|(candidate, _)| candidate == key)
        .map(|(_, value)| value.as_str())
}

fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    let mut i = 0;
    while i + 3 <= data.len() {
        let n = ((data[i] as u32) << 16) | ((data[i + 1] as u32) << 8) | data[i + 2] as u32;
        out.push(TABLE[((n >> 18) & 0x3f) as usize] as char);
        out.push(TABLE[((n >> 12) & 0x3f) as usize] as char);
        out.push(TABLE[((n >> 6) & 0x3f) as usize] as char);
        out.push(TABLE[(n & 0x3f) as usize] as char);
        i += 3;
    }
    match data.len() - i {
        1 => {
            let n = (data[i] as u32) << 16;
            out.push(TABLE[((n >> 18) & 0x3f) as usize] as char);
            out.push(TABLE[((n >> 12) & 0x3f) as usize] as char);
            out.push('=');
            out.push('=');
        }
        2 => {
            let n = ((data[i] as u32) << 16) | ((data[i + 1] as u32) << 8);
            out.push(TABLE[((n >> 18) & 0x3f) as usize] as char);
            out.push(TABLE[((n >> 12) & 0x3f) as usize] as char);
            out.push(TABLE[((n >> 6) & 0x3f) as usize] as char);
            out.push('=');
        }
        _ => {}
    }
    out
}

fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h0: u32 = 0x6745_2301;
    let mut h1: u32 = 0xefcd_ab89;
    let mut h2: u32 = 0x98ba_dcfe;
    let mut h3: u32 = 0x1032_5476;
    let mut h4: u32 = 0xc3d2_e1f0;

    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut msg = Vec::with_capacity((data.len() + 9).div_ceil(64) * 64);
    msg.extend_from_slice(data);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for block in msg.as_chunks::<64>().0.iter() {
        let mut w = [0u32; 80];
        for (i, word) in w.iter_mut().take(16).enumerate() {
            let j = i * 4;
            *word = u32::from_be_bytes([block[j], block[j + 1], block[j + 2], block[j + 3]]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for (i, wi) in w.iter().copied().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5a82_7999),
                20..=39 => (b ^ c ^ d, 0x6ed9_eba1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8f1b_bcdc),
                _ => (b ^ c ^ d, 0xca62_c1d6),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(wi);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut out = [0u8; 20];
    out[0..4].copy_from_slice(&h0.to_be_bytes());
    out[4..8].copy_from_slice(&h1.to_be_bytes());
    out[8..12].copy_from_slice(&h2.to_be_bytes());
    out[12..16].copy_from_slice(&h3.to_be_bytes());
    out[16..20].copy_from_slice(&h4.to_be_bytes());
    out
}

fn hmac_sha1(key: &[u8], data: &[u8]) -> [u8; 20] {
    const BLOCK: usize = 64;
    let mut key_block = [0u8; BLOCK];
    if key.len() > BLOCK {
        key_block[..20].copy_from_slice(&sha1(key));
    } else {
        key_block[..key.len()].copy_from_slice(key);
    }

    let mut inner = Vec::with_capacity(BLOCK + data.len());
    let mut outer = Vec::with_capacity(BLOCK + 20);
    for b in key_block {
        inner.push(b ^ 0x36);
        outer.push(b ^ 0x5c);
    }
    inner.extend_from_slice(data);
    let inner_digest = sha1(&inner);
    outer.extend_from_slice(&inner_digest);
    sha1(&outer)
}

fn hmac_base64_encode(key: &str, data: &str) -> String {
    base64_encode(&hmac_sha1(key.as_bytes(), data.as_bytes()))
}

fn oauth_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

fn oauth_nonce() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let seq = NONCE_COUNTER.fetch_add(1, Ordering::Relaxed);
    now.wrapping_add(seq).to_string()
}

fn oauth_params(consumer_key: &str, token: Option<&str>) -> Vec<(String, String)> {
    let mut params = vec![("oauth_consumer_key".to_string(), consumer_key.to_string())];
    if let Some(token) = token {
        params.push(("oauth_token".to_string(), token.to_string()));
    }
    params.push((
        "oauth_signature_method".to_string(),
        TWITTER_SIGNATURE_METHOD.to_string(),
    ));
    params.push(("oauth_timestamp".to_string(), oauth_timestamp()));
    params.push(("oauth_nonce".to_string(), oauth_nonce()));
    params.push((
        "oauth_version".to_string(),
        TWITTER_OAUTH_VERSION.to_string(),
    ));
    params
}

fn twitter_request(
    request_page: &str,
    domain: &str,
    request_method: &str,
    request_params: &[(String, String)],
    oauth_params: &[(String, String)],
    extra_headers: &[(&str, String)],
    body: &[u8],
    consumer_secret: &str,
    token_secret: &str,
) -> Result<Vec<u8>> {
    let request_address = format!("https://{domain}{request_page}");

    // `eng_twitter.cpp` signs request parameters together with OAuth parameters,
    // sorts by key only, then places that complete list in the Authorization
    // header.  This is intentionally preserved, including request params in the
    // OAuth header and query string for POST requests.
    let mut authorization_params =
        Vec::with_capacity(request_params.len() + oauth_params.len() + 1);
    authorization_params.extend_from_slice(request_params);
    authorization_params.extend_from_slice(oauth_params);
    authorization_params.sort_by(|a, b| a.0.cmp(&b.0));

    let signature_param = authorization_params
        .iter()
        .map(|(key, value)| format!("{}={}", percent_encode(key), percent_encode(value)))
        .collect::<Vec<_>>()
        .join("&");
    let signature_key = format!(
        "{}&{}",
        percent_encode(consumer_secret),
        percent_encode(token_secret)
    );
    let signature_value = format!(
        "{}&{}&{}",
        percent_encode(request_method),
        percent_encode(&request_address),
        percent_encode(&signature_param)
    );
    let signature = hmac_base64_encode(&signature_key, &signature_value);
    authorization_params.push(("oauth_signature".to_string(), signature));

    let auth_header = format!(
        "OAuth {}",
        authorization_params
            .iter()
            .map(|(key, value)| format!("{}=\"{}\"", percent_encode(key), percent_encode(value)))
            .collect::<Vec<_>>()
            .join(",")
    );

    let query = request_params
        .iter()
        .map(|(key, value)| format!("{}={}", percent_encode(key), percent_encode(value)))
        .collect::<Vec<_>>()
        .join("&");
    let url = if query.is_empty() {
        request_address
    } else {
        format!("{request_address}?{query}")
    };

    let mut request = match request_method {
        "GET" => ureq::get(&url),
        "POST" => ureq::post(&url),
        other => bail!("unsupported Twitter HTTP method {other}"),
    };
    request = request
        .set("User-Agent", TWITTER_USER_AGENT)
        .set("Authorization", &auth_header);
    for (name, value) in extra_headers {
        request = request.set(name, value);
    }

    let response = if request_method == "POST" && !body.is_empty() {
        match request.send_bytes(body) {
            Ok(response) => response,
            Err(ureq::Error::Status(_, response)) => response,
            Err(err) => return Err(anyhow!(err)).with_context(|| format!("POST {url}")),
        }
    } else {
        match request.call() {
            Ok(response) => response,
            Err(ureq::Error::Status(_, response)) => response,
            Err(err) => {
                return Err(anyhow!(err)).with_context(|| format!("{request_method} {url}"));
            }
        }
    };

    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .with_context(|| format!("read Twitter response from {url}"))?;
    // WinINet's original twitter_request() returned success once the response
    // body was read, regardless of the HTTP status.  Leave semantic errors to
    // the caller's OAuth/JSON parsing for parity.
    Ok(bytes)
}

fn persist_state(project_dir: &Path, state: &TwitterState) -> Result<()> {
    let path = state_path(project_dir);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create Twitter state directory {}", parent.display()))?;
    }
    let values = [
        ("oauth_access_token", state.access_token.as_str()),
        (
            "oauth_access_token_secret",
            state.access_token_secret.as_str(),
        ),
        ("twitter_user_id", state.user_id.as_str()),
        ("twitter_user_name", state.user_name.as_str()),
        ("twitter_screen_name", state.screen_name.as_str()),
    ];
    let mut body = String::new();
    for (key, value) in values {
        body.push_str(key);
        body.push('=');
        body.push_str(&percent_encode(value));
        body.push('\n');
    }
    fs::write(&path, body).with_context(|| format!("write Twitter state {}", path.display()))
}

pub fn save_state(ctx: &CommandContext) -> Result<()> {
    persist_state(&ctx.project_dir, &ctx.globals.twitter)
}

pub fn ensure_state_loaded(ctx: &mut CommandContext) {
    if ctx.globals.twitter.loaded {
        return;
    }
    ctx.globals.twitter.loaded = true;
    let path = state_path(&ctx.project_dir);
    let Ok(text) = fs::read_to_string(&path) else {
        return;
    };
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = percent_decode(value);
        match key {
            "oauth_access_token" => ctx.globals.twitter.access_token = value,
            "oauth_access_token_secret" => ctx.globals.twitter.access_token_secret = value,
            "twitter_user_id" => ctx.globals.twitter.user_id = value,
            "twitter_user_name" => ctx.globals.twitter.user_name = value,
            "twitter_screen_name" => ctx.globals.twitter.screen_name = value,
            _ => {}
        }
    }
}

pub fn begin_authorize(ctx: &mut CommandContext) -> Result<String> {
    ensure_state_loaded(ctx);
    ctx.globals.twitter.clear_authorization();

    let consumer_key = gameexe_value(ctx, "TWITTER.API_KEY");
    let consumer_secret = gameexe_value(ctx, "TWITTER.API_SECRET");
    let callback_url = gameexe_value(ctx, "TWITTER.CALLBACK_URL");
    if consumer_key.is_empty() || consumer_secret.is_empty() || callback_url.is_empty() {
        bail!("Twitter API_KEY, API_SECRET and CALLBACK_URL must all be configured");
    }

    let mut oauth = oauth_params(&consumer_key, None);
    oauth.insert(0, ("oauth_callback".to_string(), callback_url));
    let response = twitter_request(
        "/oauth/request_token",
        TWITTER_API_DOMAIN,
        "POST",
        &[],
        &oauth,
        &[],
        &[],
        &consumer_secret,
        "",
    )?;
    let response = String::from_utf8_lossy(&response);
    let pairs = query_to_pairs(&response);
    let request_token = query_value(&pairs, "oauth_token")
        .ok_or_else(|| anyhow!("Twitter request-token response omitted oauth_token"))?
        .to_string();
    let request_secret = query_value(&pairs, "oauth_token_secret")
        .ok_or_else(|| anyhow!("Twitter request-token response omitted oauth_token_secret"))?
        .to_string();
    ctx.globals.twitter.request_token = request_token.clone();
    ctx.globals.twitter.request_token_secret = request_secret;

    let auth_url = format!(
        "https://api.twitter.com/oauth/authenticate?oauth_token={}",
        percent_encode(&request_token)
    );
    ctx.net.open_url(&auth_url)?;
    Ok(auth_url)
}

fn callback_query<'a>(configured_callback: &str, submitted: &'a str) -> Result<&'a str> {
    let trimmed = submitted.trim();
    if trimmed.is_empty() {
        bail!("OAuth callback/verifier is empty");
    }

    // The original embedded browser accepts a navigation only when it begins
    // with #TWITTER.CALLBACK_URL, irrespective of URL scheme. Preserve that
    // rule for pasted callbacks (including custom schemes). Desktop ports also
    // accept a bare verifier because the system browser cannot be intercepted.
    if !configured_callback.is_empty() && trimmed.starts_with(configured_callback) {
        let tail = trimmed
            .strip_prefix(configured_callback)
            .ok_or_else(|| anyhow!("OAuth callback URL does not match #TWITTER.CALLBACK_URL"))?;
        // CIESink::OnBeforeNavigate2 passes response.substr(callback.size()+1),
        // i.e. it consumes exactly the one separator byte after the callback.
        return tail
            .get(1..)
            .filter(|query| !query.is_empty())
            .ok_or_else(|| anyhow!("OAuth callback URL has no query string"));
    }
    if trimmed.contains("://") {
        bail!("OAuth callback URL does not match #TWITTER.CALLBACK_URL");
    }
    Ok(trimmed)
}

pub fn complete_authorize(
    ctx: &mut CommandContext,
    submitted_callback_or_verifier: &str,
) -> Result<()> {
    ensure_state_loaded(ctx);
    let consumer_key = gameexe_value(ctx, "TWITTER.API_KEY");
    let consumer_secret = gameexe_value(ctx, "TWITTER.API_SECRET");
    let callback_url = gameexe_value(ctx, "TWITTER.CALLBACK_URL");
    if consumer_key.is_empty() || consumer_secret.is_empty() || callback_url.is_empty() {
        bail!("Twitter API_KEY, API_SECRET and CALLBACK_URL must all be configured");
    }
    if ctx.globals.twitter.request_token.is_empty()
        || ctx.globals.twitter.request_token_secret.is_empty()
    {
        bail!("Twitter request token is missing; start authentication again");
    }

    let query = callback_query(&callback_url, submitted_callback_or_verifier)?;
    let pairs = query_to_pairs(query);
    let bare_verifier = !query.contains('=') && !query.contains('&');
    let verifier = query_value(&pairs, "oauth_verifier")
        .map(str::to_string)
        .or_else(|| bare_verifier.then(|| query.to_string()))
        .ok_or_else(|| anyhow!("OAuth callback omitted oauth_verifier"))?;
    let returned_token = if bare_verifier {
        ctx.globals.twitter.request_token.clone()
    } else {
        // Original query["oauth_token"] yields an empty string when omitted; it
        // does not compare this token with G_oauth_request_token.
        query_value(&pairs, "oauth_token").unwrap_or("").to_string()
    };

    let mut oauth = oauth_params(&consumer_key, Some(&returned_token));
    oauth.insert(2, ("oauth_verifier".to_string(), verifier));
    let response = twitter_request(
        "/oauth/access_token",
        TWITTER_API_DOMAIN,
        "POST",
        &[],
        &oauth,
        &[],
        &[],
        &consumer_secret,
        &ctx.globals.twitter.request_token_secret,
    )?;
    let response = String::from_utf8_lossy(&response);
    let pairs = query_to_pairs(&response);
    ctx.globals.twitter.access_token = query_value(&pairs, "oauth_token")
        .ok_or_else(|| anyhow!("Twitter access-token response omitted oauth_token"))?
        .to_string();
    ctx.globals.twitter.access_token_secret = query_value(&pairs, "oauth_token_secret")
        .ok_or_else(|| anyhow!("Twitter access-token response omitted oauth_token_secret"))?
        .to_string();
    ctx.globals.twitter.user_id = query_value(&pairs, "user_id")
        .ok_or_else(|| anyhow!("Twitter access-token response omitted user_id"))?
        .to_string();
    ctx.globals.twitter.screen_name = query_value(&pairs, "screen_name")
        .ok_or_else(|| anyhow!("Twitter access-token response omitted screen_name"))?
        .to_string();

    let oauth = oauth_params(&consumer_key, Some(&ctx.globals.twitter.access_token));
    let request = vec![("user_id".to_string(), ctx.globals.twitter.user_id.clone())];
    let response = twitter_request(
        "/1.1/users/show.json",
        TWITTER_API_DOMAIN,
        "GET",
        &request,
        &oauth,
        &[],
        &[],
        &consumer_secret,
        &ctx.globals.twitter.access_token_secret,
    )?;
    let response_text = String::from_utf8_lossy(&response);
    ctx.globals.twitter.user_name = json_string_field(&response_text, "name")
        .ok_or_else(|| anyhow!("Twitter users/show response omitted name"))?;

    ctx.globals.twitter.request_token.clear();
    ctx.globals.twitter.request_token_secret.clear();
    Ok(())
}

pub fn tweet(ctx: &mut CommandContext, text: &str, image_path: &Path) -> Result<()> {
    ensure_state_loaded(ctx);
    if !ctx.globals.twitter.is_authorized() {
        bail!("Twitter is not authorized");
    }
    let consumer_key = gameexe_value(ctx, "TWITTER.API_KEY");
    let consumer_secret = gameexe_value(ctx, "TWITTER.API_SECRET");
    if consumer_key.is_empty() || consumer_secret.is_empty() {
        bail!("Twitter API_KEY and API_SECRET must be configured");
    }

    let image = fs::read(image_path)
        .with_context(|| format!("read tweet image {}", image_path.display()))?;
    let image_base64 = base64_encode(&image);
    let boundary = format!("S-i-g-l-u-s--{}{}", oauth_nonce(), oauth_nonce());
    let body = format!(
        "--{boundary}\r\nContent-Disposition: form-data; name=\"media_data\"; \r\n\r\n{image_base64}\r\n--{boundary}--\r\n\r\n"
    );
    let oauth = oauth_params(&consumer_key, Some(&ctx.globals.twitter.access_token));
    let response = twitter_request(
        "/1.1/media/upload.json",
        TWITTER_UPLOAD_DOMAIN,
        "POST",
        &[],
        &oauth,
        &[(
            "Content-Type",
            format!("multipart/form-data; boundary={boundary}"),
        )],
        body.as_bytes(),
        &consumer_secret,
        &ctx.globals.twitter.access_token_secret,
    )?;
    let response_text = String::from_utf8_lossy(&response);
    let media_id = json_string_field(&response_text, "media_id_string")
        .ok_or_else(|| anyhow!("Twitter media upload response omitted media_id_string"))?;

    let oauth = oauth_params(&consumer_key, Some(&ctx.globals.twitter.access_token));
    let request = vec![
        ("status".to_string(), text.to_string()),
        ("media_ids".to_string(), media_id),
    ];
    let response = twitter_request(
        "/1.1/statuses/update.json",
        TWITTER_API_DOMAIN,
        "POST",
        &request,
        &oauth,
        &[],
        &[],
        &consumer_secret,
        &ctx.globals.twitter.access_token_secret,
    )?;
    let response_text = String::from_utf8_lossy(&response);
    if let Some(message) = json_error_message(&response_text) {
        bail!("Twitter posting failed: {message}");
    }
    if serde_json::from_str::<serde_json::Value>(&response_text)
        .ok()
        .and_then(|value| value.get("errors").cloned())
        .is_some()
    {
        bail!("Twitter posting failed: Twitter returned an unspecified posting error");
    }
    Ok(())
}

fn json_string_field(text: &str, field: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    value.get(field)?.as_str().map(str::to_string)
}

fn json_error_message(text: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    value
        .get("errors")?
        .as_array()?
        .first()?
        .get("message")?
        .as_str()
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_encoding_matches_oauth_rules() {
        assert_eq!(
            percent_encode("Ladies + Gentlemen"),
            "Ladies%20%2B%20Gentlemen"
        );
        assert_eq!(percent_encode("☃"), "%E2%98%83");
        assert_eq!(percent_decode("a%20b%2Bc"), "a b+c");
    }

    #[test]
    fn base64_vectors() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
    }

    #[test]
    fn sha1_vector() {
        let digest = sha1(b"abc");
        assert_eq!(
            digest,
            [
                0xa9, 0x99, 0x3e, 0x36, 0x47, 0x06, 0x81, 0x6a, 0xba, 0x3e, 0x25, 0x71, 0x78, 0x50,
                0xc2, 0x6c, 0x9c, 0xd0, 0xd8, 0x9d,
            ]
        );
    }

    #[test]
    fn hmac_sha1_vector() {
        // RFC 2202 test case 2.
        assert_eq!(
            hmac_base64_encode("Jefe", "what do ya want for nothing?"),
            "7/zfauXrL6LSdBbV8YTfnCWafHk="
        );
    }

    #[test]
    fn json_field_handles_escapes() {
        assert_eq!(
            json_string_field(r#"{"name":"Siglus \u30c6\u30b9\u30c8"}"#, "name").as_deref(),
            Some("Siglus テスト")
        );
    }
}
