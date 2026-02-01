use rand::Rng;
use serde::de::DeserializeOwned;
use tracing::{debug, info, warn};

use crate::error::{ItaError, Result};

const BATCH_URL: &str = "https://content-alkalimatrix-pa.googleapis.com/batch";

const INNER_HEADERS: &str = "\
x-alkali-application-key: applications/matrix\r\n\
x-alkali-auth-apps-namespace: alkali_v2\r\n\
x-alkali-auth-entities-namespace: alkali_v2\r\n\
X-JavaScript-User-Agent: google-api-javascript-client/1.1.0\r\n\
X-Requested-With: XMLHttpRequest\r\n\
X-Goog-Encode-Response-If-Executable: base64\r\n\
X-ClientDetails: appVersion=5.0%20(X11%3B%20Linux%20x86_64)%20AppleWebKit%2F537.36%20\
(KHTML%2C%20like%20Gecko)%20Chrome%2F144.0.0.0%20Safari%2F537.36\
&platform=Linux%20x86_64\
&userAgent=Mozilla%2F5.0%20(X11%3B%20Linux%20x86_64)%20AppleWebKit%2F537.36%20\
(KHTML%2C%20like%20Gecko)%20Chrome%2F144.0.0.0%20Safari%2F537.36";

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
    (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36";

pub struct BatchTransport {
    client: reqwest::Client,
    api_key: String,
}

impl BatchTransport {
    pub fn new(api_key: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .build()
            .map_err(ItaError::Transport)?;

        Ok(Self {
            client,
            api_key: api_key.to_string(),
        })
    }

    fn gen_batch_id() -> String {
        let mut rng = rand::rng();
        let id: u64 = rng.random_range(100_000_000_000_000_000..=u64::MAX);
        id.to_string()
    }

    fn build_inner_request(method: &str, path: &str, body: Option<&str>) -> String {
        let mut inner = format!("{method} {path}\r\n");
        inner.push_str(INNER_HEADERS);
        inner.push_str("\r\n");

        if let Some(json_body) = body {
            inner.push_str("Content-Type: application/json\r\n");
            inner.push_str(&format!("Content-Length: {}\r\n", json_body.len()));
            inner.push_str("\r\n");
            inner.push_str(json_body);
        }

        inner
    }

    fn build_envelope(method: &str, path: &str, body: Option<&str>) -> (String, String) {
        let batch_id = Self::gen_batch_id();

        let ct_value = format!("multipart/mixed; boundary=batch{batch_id}");
        let ct_encoded = ct_value
            .replace('/', "%2F")
            .replace(';', "%3B")
            .replace(' ', "%20")
            .replace('=', "%3D");

        let url = format!("{BATCH_URL}?%24ct={ct_encoded}");

        let inner = Self::build_inner_request(method, path, body);
        let multipart_body = format!(
            "--batch{batch_id}\r\n\
             Content-Type: application/http\r\n\
             Content-Transfer-Encoding: binary\r\n\
             Content-ID: <batch{batch_id}+gapiRequest@googleapis.com>\r\n\
             \r\n\
             {inner}\r\n\
             --batch{batch_id}--"
        );

        (url, multipart_body)
    }

    fn parse_inner_response(raw: &str) -> Result<(u16, &str)> {
        let status = raw
            .lines()
            .find(|line| line.starts_with("HTTP/"))
            .and_then(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                parts.get(1)?.parse::<u16>().ok()
            })
            .ok_or(ItaError::BatchParse {
                reason: "no HTTP status line in inner response",
            })?;

        let json_start = raw.find("\r\n\r\n").ok_or(ItaError::BatchParse {
            reason: "no header/body separator in inner response",
        })?;

        let after_headers = &raw[json_start..];
        let obj_start = after_headers.find('{').ok_or(ItaError::BatchParse {
            reason: "no JSON object in inner response body",
        })?;

        let json_str = &after_headers[obj_start..];
        let json_str = json_str
            .rfind("\r\n--")
            .map(|i| &json_str[..i])
            .unwrap_or(json_str)
            .trim();

        Ok((status, json_str))
    }

    pub fn path_with_key(&self, path: &str) -> String {
        if path.contains('?') {
            format!("{path}&key={}", self.api_key)
        } else {
            format!("{path}?key={}", self.api_key)
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let full_path = self.path_with_key(path);
        self.send("GET", &full_path, None).await
    }

    pub async fn post<T: DeserializeOwned>(&self, path: &str, body: &str) -> Result<T> {
        let full_path = format!("{}&alt=json", self.path_with_key(path));
        self.send("POST", &full_path, Some(body)).await
    }

    async fn send<T: DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<T> {
        let (url, multipart_body) = Self::build_envelope(method, path, body);

        info!("{method} {path}");
        if let Some(b) = body {
            debug!(body_len = b.len(), "request body");
        }

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "text/plain; charset=UTF-8")
            .header("Origin", "https://matrix.itasoftware.com")
            .header("Referer", "https://matrix.itasoftware.com/")
            .header("X-Client-Data", "CIKBywE=")
            .header("Accept", "*/*")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("User-Agent", USER_AGENT)
            .body(multipart_body)
            .send()
            .await
            .map_err(ItaError::Transport)?;

        let outer_status = resp.status().as_u16();
        let text = resp.text().await.map_err(ItaError::Transport)?;

        let (inner_status, json_str) = Self::parse_inner_response(&text)?;

        info!(outer_status, inner_status, "response");
        debug!(json = json_str, "response body");

        if inner_status != 200 {
            warn!(
                outer_status,
                inner_status,
                json = json_str,
                "non-200 inner status"
            );
            return Err(ItaError::UnexpectedStatus {
                outer: outer_status,
                inner: inner_status,
            });
        }

        serde_json::from_str(json_str).map_err(ItaError::Json)
    }

    pub async fn post_raw(&self, path: &str, body: &str) -> Result<(u16, String)> {
        let full_path = format!("{}&alt=json", self.path_with_key(path));
        let (url, multipart_body) = Self::build_envelope("POST", &full_path, Some(body));

        info!("POST {}", full_path);

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "text/plain; charset=UTF-8")
            .header("Origin", "https://matrix.itasoftware.com")
            .header("Referer", "https://matrix.itasoftware.com/")
            .header("X-Client-Data", "CIKBywE=")
            .header("Accept", "*/*")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("User-Agent", USER_AGENT)
            .body(multipart_body)
            .send()
            .await
            .map_err(ItaError::Transport)?;

        let text = resp.text().await.map_err(ItaError::Transport)?;
        let (inner_status, json_str) = Self::parse_inner_response(&text)?;

        info!(inner_status, "raw response");
        debug!(json = json_str, "raw response body");

        Ok((inner_status, json_str.to_string()))
    }
}
