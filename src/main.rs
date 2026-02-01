use anyhow::{Context, Result};
use rand::Rng;
use serde_json::Value;

const API_KEY: &str = "AIzaSyBH1mte6BdKzvf0c2mYprkyvfHCRWmfX7g";
const BATCH_URL: &str = "https://content-alkalimatrix-pa.googleapis.com/batch";

/// Generate a random batch ID matching the format seen in captured traffic
/// (large positive integer as string).
fn gen_batch_id() -> String {
    let mut rng = rand::rng();
    let id: u64 = rng.random_range(100_000_000_000_000_000..=u64::MAX);
    id.to_string()
}

/// Build the inner HTTP request that goes inside the multipart batch body.
/// This replicates the exact format from captured browser traffic.
fn build_inner_request(method: &str, path: &str, body: Option<&str>) -> String {
    assert!(!method.is_empty(), "method must not be empty");
    assert!(!path.is_empty(), "path must not be empty");

    let mut inner = format!("{method} {path}\r\n");

    // Inner headers — exact order and values from captured traffic
    inner.push_str("x-alkali-application-key: applications/matrix\r\n");
    inner.push_str("x-alkali-auth-apps-namespace: alkali_v2\r\n");
    inner.push_str("x-alkali-auth-entities-namespace: alkali_v2\r\n");
    inner.push_str("X-JavaScript-User-Agent: google-api-javascript-client/1.1.0\r\n");
    inner.push_str("X-Requested-With: XMLHttpRequest\r\n");
    inner.push_str("X-Goog-Encode-Response-If-Executable: base64\r\n");
    inner.push_str("X-ClientDetails: appVersion=5.0%20(X11%3B%20Linux%20x86_64)%20AppleWebKit%2F537.36%20(KHTML%2C%20like%20Gecko)%20Chrome%2F144.0.0.0%20Safari%2F537.36&platform=Linux%20x86_64&userAgent=Mozilla%2F5.0%20(X11%3B%20Linux%20x86_64)%20AppleWebKit%2F537.36%20(KHTML%2C%20like%20Gecko)%20Chrome%2F144.0.0.0%20Safari%2F537.36\r\n");

    if let Some(json_body) = body {
        inner.push_str("Content-Type: application/json\r\n");
        inner.push_str(&format!("Content-Length: {}\r\n", json_body.len()));
        inner.push_str("\r\n");
        inner.push_str(json_body);
    }

    inner
}

/// Wrap an inner HTTP request in the GAPI multipart batch envelope.
/// Returns (full_url, multipart_body).
fn build_batch_request(method: &str, path: &str, body: Option<&str>) -> (String, String) {
    let batch_id = gen_batch_id();

    // The $ct query param carries the real multipart content type
    let ct_value = format!("multipart/mixed; boundary=batch{batch_id}");
    let ct_encoded = ct_value
        .replace('/', "%2F")
        .replace(';', "%3B")
        .replace(' ', "%20")
        .replace('=', "%3D");

    let url = format!("{BATCH_URL}?%24ct={ct_encoded}");

    // Build multipart body — exact format from captured traffic
    let inner = build_inner_request(method, path, body);
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

/// Parse the inner HTTP response from a multipart batch response body.
/// Returns (inner_status, inner_body_json).
fn parse_batch_response(raw: &str) -> Result<(u16, Option<Value>)> {
    // Find the inner HTTP status line (e.g. "HTTP/1.1 200 OK")
    let status = raw
        .lines()
        .find(|line| line.starts_with("HTTP/"))
        .and_then(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            parts.get(1)?.parse::<u16>().ok()
        })
        .unwrap_or(0);

    // Find JSON body — starts after the first blank line following headers
    // Look for the first '{' or '[' that starts a JSON block
    let json_body = if let Some(json_start) = raw.find("\r\n\r\n") {
        let after_headers = &raw[json_start..];
        // Find the actual JSON start
        if let Some(obj_start) = after_headers.find('{') {
            let json_str = &after_headers[obj_start..];
            // Trim trailing multipart boundary
            let json_str = json_str
                .rfind("\r\n--")
                .map(|i| &json_str[..i])
                .unwrap_or(json_str)
                .trim();
            serde_json::from_str(json_str).ok()
        } else {
            None
        }
    } else {
        None
    };

    Ok((status, json_body))
}

/// Send a batch request and return the parsed inner response.
async fn send_batch(
    client: &reqwest::Client,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<(u16, String, Option<Value>)> {
    let (url, multipart_body) = build_batch_request(method, path, body);

    println!("  POST {url}");
    println!("  Inner: {method} {path}");
    if let Some(b) = body {
        let preview_len = b.len().min(200);
        println!("  Body preview: {}...", &b[..preview_len]);
    }

    let resp = client
        .post(&url)
        .header("Content-Type", "text/plain; charset=UTF-8")
        .header("Origin", "https://matrix.itasoftware.com")
        .header("Referer", "https://matrix.itasoftware.com/")
        .header("X-Client-Data", "CIKBywE=")
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36",
        )
        .body(multipart_body)
        .send()
        .await
        .context("failed to send batch request")?;

    let outer_status = resp.status().as_u16();
    let text = resp.text().await.context("failed to read response body")?;

    let (inner_status, json) = parse_batch_response(&text).unwrap_or((0, None));

    println!("  Outer HTTP: {outer_status} | Inner HTTP: {inner_status}");

    // Print a useful preview of the JSON response
    if let Some(ref j) = json {
        let pretty = serde_json::to_string_pretty(j).unwrap_or_else(|_| j.to_string());
        let preview_len = pretty.len().min(1500);
        println!("  Response JSON preview:\n{}", &pretty[..preview_len]);
        if pretty.len() > 1500 {
            println!("  ... ({} more chars)", pretty.len() - 1500);
        }
    } else if outer_status != 200 {
        let preview_len = text.len().min(1000);
        println!("  Raw response preview:\n{}", &text[..preview_len]);
    }
    println!();

    Ok((inner_status, text, json))
}

/// Build the search request body matching the exact format from captured browser traffic.
/// If `bg_program_response` is None, the field is omitted (to test if it's required).
fn build_search_body(
    origins: &[&str],
    destinations: &[&str],
    date: &str,
    bg_program_response: Option<&str>,
) -> Value {
    assert!(!origins.is_empty(), "origins must not be empty");
    assert!(!destinations.is_empty(), "destinations must not be empty");
    assert!(!date.is_empty(), "date must not be empty");

    let origins_json: Vec<Value> = origins
        .iter()
        .map(|s| Value::String(s.to_string()))
        .collect();
    let dests_json: Vec<Value> = destinations
        .iter()
        .map(|s| Value::String(s.to_string()))
        .collect();

    let mut body = serde_json::json!({
        "summarizers": [
            "carrierStopMatrix",
            "currencyNotice",
            "solutionList",
            "itineraryPriceSlider",
            "itineraryCarrierList",
            "itineraryDepartureTimeRanges",
            "itineraryArrivalTimeRanges",
            "durationSliderItinerary",
            "itineraryOrigins",
            "itineraryDestinations",
            "itineraryStopCountList",
            "warningsItinerary"
        ],
        "inputs": {
            "filter": {},
            "page": {"current": 1, "size": 25},
            "pax": {"adults": 1},
            "slices": [{
                "origins": origins_json,
                "destinations": dests_json,
                "date": date,
                "dateModifier": {"minus": 0, "plus": 0},
                "isArrivalDate": false,
                "filter": {"warnings": {"values": []}},
                "selected": false
            }],
            "firstDayOfWeek": "SUNDAY",
            "internalUser": false,
            "sliceIndex": 0,
            "sorts": "default",
            "cabin": "COACH",
            "maxLegsRelativeToMin": 1,
            "changeOfAirport": true,
            "checkAvailability": true
        },
        "summarizerSet": "wholeTrip",
        "name": "specificDatesSlice"
    });

    // Add bgProgramResponse if provided
    if let Some(token) = bg_program_response {
        body.as_object_mut()
            .expect("body must be an object")
            .insert(
                "bgProgramResponse".to_string(),
                Value::String(token.to_string()),
            );
    }

    body
}

/// Build a summarize request body (for getting details after a search).
fn build_summarize_body(solution_set: &str, session: &str, solution_id: Option<&str>) -> Value {
    assert!(!solution_set.is_empty(), "solution_set must not be empty");
    assert!(!session.is_empty(), "session must not be empty");

    let summarizers = if solution_id.is_some() {
        // Booking details request
        serde_json::json!(["bookingDetails"])
    } else {
        // General summarize request
        serde_json::json!([
            "carrierStopMatrix",
            "currencyNotice",
            "solutionList",
            "itineraryPriceSlider",
            "itineraryCarrierList",
            "itineraryDepartureTimeRanges",
            "itineraryArrivalTimeRanges",
            "durationSliderItinerary",
            "itineraryOrigins",
            "itineraryDestinations",
            "itineraryStopCountList",
            "warningsItinerary"
        ])
    };

    let summarizer_set = if solution_id.is_some() {
        "viewDetails"
    } else {
        "wholeTrip"
    };

    let mut inputs = serde_json::json!({
        "filter": {},
        "page": {"current": 1, "size": 25},
        "pax": {"adults": 1},
        "slices": [],
        "firstDayOfWeek": "SUNDAY",
        "internalUser": false,
        "sliceIndex": 0,
        "sorts": "default",
        "cabin": "COACH",
        "maxLegsRelativeToMin": 1,
        "changeOfAirport": true,
        "checkAvailability": true
    });

    if let Some(sol) = solution_id {
        inputs
            .as_object_mut()
            .expect("inputs must be object")
            .insert("solution".to_string(), Value::String(sol.to_string()));
    }

    serde_json::json!({
        "summarizers": summarizers,
        "inputs": inputs,
        "summarizerSet": summarizer_set,
        "solutionSet": solution_set,
        "session": session
    })
}

/// Extract solutionSet and session from a search response JSON.
fn extract_search_session(json: &Value) -> Option<(String, String)> {
    let result = json.as_object()?;
    let solution_set = result.get("solutionSet")?.as_str()?.to_string();
    let session = result.get("session")?.as_str()?.to_string();
    Some((solution_set, session))
}

/// Extract the first solution ID from a search response.
fn extract_first_solution_id(json: &Value) -> Option<String> {
    let result = json.as_object()?;
    // Navigate: result.data.solutionList.solutions[0].id
    let data = result.get("data")?.as_object()?;
    let solution_list = data.get("solutionList")?.as_object()?;
    let solutions = solution_list.get("solutions")?.as_array()?;
    let first = solutions.first()?.as_object()?;
    Some(first.get("id")?.as_str()?.to_string())
}

fn print_banner(text: &str) {
    let width = text.len() + 4;
    let border: String = "═".repeat(width);
    println!("╔{border}╗");
    println!("║  {text}  ║");
    println!("╚{border}╝");
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::builder()
        .build()
        .context("failed to build HTTP client")?;

    // ============================================================
    // Test 1: Location autocomplete (simplest GET, known to work)
    // ============================================================
    print_banner("Test 1: Location autocomplete (GET)");

    let path = format!(
        "/v1/locationTypes/CITIES_AND_AIRPORTS/partialNames/PHX/locations?pageSize=10&key={API_KEY}"
    );
    let (status, _, _) = send_batch(&client, "GET", &path, None).await?;

    if status == 200 {
        println!("  >>> PASS <<<\n");
    } else {
        println!("  >>> FAIL (status {status}) <<<\n");
    }

    // ============================================================
    // Test 2: Currency list (another simple GET)
    // ============================================================
    print_banner("Test 2: Currency list (GET)");

    let path = format!("/v1/currencies?key={API_KEY}");
    let (status, _, _) = send_batch(&client, "GET", &path, None).await?;

    if status == 200 {
        println!("  >>> PASS <<<\n");
    } else {
        println!("  >>> FAIL (status {status}) <<<\n");
    }

    // ============================================================
    // Test 3: Airport code lookup — fixed to use airportOrMultiAirportCity
    // ============================================================
    print_banner("Test 3: Airport lookup (GET) — airportOrMultiAirportCity");

    let path =
        format!("/v1/locationTypes/airportOrMultiAirportCity/locationCodes/PHX?key={API_KEY}");
    let (status, _, _) = send_batch(&client, "GET", &path, None).await?;

    if status == 200 {
        println!("  >>> PASS <<<\n");
    } else {
        println!("  >>> FAIL (status {status}) <<<\n");
        // Fallback: try original AIRPORT type
        println!("  Trying fallback: AIRPORT...");
        let path = format!("/v1/locationTypes/AIRPORT/locationCodes/PHX?key={API_KEY}");
        let (status, _, _) = send_batch(&client, "GET", &path, None).await?;
        if status == 200 {
            println!("  >>> Fallback PASS (AIRPORT type works too) <<<\n");
        } else {
            println!("  >>> Fallback FAIL (status {status}) <<<\n");
        }
    }

    // ============================================================
    // Test 4a: Flight search WITHOUT bgProgramResponse
    //          (testing if it's required or optional)
    // ============================================================
    print_banner("Test 4a: Flight search — NO bgProgramResponse");

    let search_body = build_search_body(
        &["PHX"],
        &["DTW"],
        "2026-02-15",
        None, // no bgProgramResponse
    );
    let search_json =
        serde_json::to_string(&search_body).context("failed to serialize search body")?;

    let path = format!("/v1/search?key={API_KEY}&alt=json");
    let (status, _, json_4a) = send_batch(&client, "POST", &path, Some(&search_json)).await?;

    let mut solution_set: Option<String> = None;
    let mut session: Option<String> = None;
    let mut first_solution_id: Option<String> = None;

    if status == 200 {
        println!("  >>> PASS — bgProgramResponse NOT required! <<<\n");
        if let Some(ref j) = json_4a {
            if let Some((ss, sess)) = extract_search_session(j) {
                println!("  solutionSet: {ss}");
                println!("  session: {sess}");
                solution_set = Some(ss);
                session = Some(sess);
            }
            if let Some(sol_id) = extract_first_solution_id(j) {
                println!("  First solution ID: {sol_id}");
                first_solution_id = Some(sol_id);
            }
        }
    } else {
        println!("  >>> FAIL (status {status}) — bgProgramResponse may be required <<<\n");
        // Print the error details if available
        if let Some(ref j) = json_4a {
            if let Some(error) = j.get("error") {
                println!(
                    "  Error: {}",
                    serde_json::to_string_pretty(error).unwrap_or_default()
                );
            }
        }
    }

    // ============================================================
    // Test 4b: Flight search WITH a bgProgramResponse token
    //          Using the captured token from batch_38 to test
    // ============================================================
    // Truncated bgProgramResponse from a real session — likely expired/session-bound
    let captured_bg_token = "!qqmlqfHNAAY";

    print_banner("Test 4b: Flight search — WITH bgProgramResponse");

    let search_body = build_search_body(&["PHX"], &["DTW"], "2026-02-15", Some(captured_bg_token));
    let search_json =
        serde_json::to_string(&search_body).context("failed to serialize search body")?;

    let path = format!("/v1/search?key={API_KEY}&alt=json");
    let (status, _, json_4b) = send_batch(&client, "POST", &path, Some(&search_json)).await?;

    if status == 200 {
        println!("  >>> PASS <<<\n");
        // If 4a failed but 4b passed, we know the token is required
        if let Some(ref j) = json_4b {
            if let Some((ss, sess)) = extract_search_session(j) {
                if solution_set.is_none() {
                    println!("  solutionSet: {ss}");
                    println!("  session: {sess}");
                    solution_set = Some(ss);
                    session = Some(sess);
                }
            }
            if let Some(sol_id) = extract_first_solution_id(j) {
                if first_solution_id.is_none() {
                    println!("  First solution ID: {sol_id}");
                    first_solution_id = Some(sol_id);
                }
            }
        }
    } else {
        println!("  >>> FAIL (status {status}) <<<\n");
        if let Some(ref j) = json_4b {
            if let Some(error) = j.get("error") {
                println!(
                    "  Error: {}",
                    serde_json::to_string_pretty(error).unwrap_or_default()
                );
            }
        }
    }

    // ============================================================
    // Test 5: Summarize (only if we got a solutionSet from search)
    // ============================================================
    if let (Some(ss), Some(sess)) = (&solution_set, &session) {
        print_banner("Test 5: Summarize (re-query search results)");

        let summarize_body = build_summarize_body(ss, sess, None);
        let summarize_json =
            serde_json::to_string(&summarize_body).context("failed to serialize summarize body")?;

        let path = format!("/v1/summarize?key={API_KEY}&alt=json");
        let (status, _, _) = send_batch(&client, "POST", &path, Some(&summarize_json)).await?;

        if status == 200 {
            println!("  >>> PASS <<<\n");
        } else {
            println!("  >>> FAIL (status {status}) <<<\n");
        }

        // ============================================================
        // Test 6: Booking details (only if we got a solution ID)
        // ============================================================
        if let Some(ref sol_id) = first_solution_id {
            print_banner("Test 6: Booking details (viewDetails)");

            let details_body = build_summarize_body(ss, sess, Some(sol_id));
            let details_json =
                serde_json::to_string(&details_body).context("failed to serialize details body")?;

            let path = format!("/v1/summarize?key={API_KEY}&alt=json");
            let (status, _, _) = send_batch(&client, "POST", &path, Some(&details_json)).await?;

            if status == 200 {
                println!("  >>> PASS <<<\n");
            } else {
                println!("  >>> FAIL (status {status}) <<<\n");
            }
        }
    } else {
        println!("\n  ⚠ Skipping Tests 5-6 (no solutionSet/session from search)\n");
    }

    // ============================================================
    // Summary
    // ============================================================
    println!("╔══════════════════════════════════════════════════╗");
    println!("║                    SUMMARY                       ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!("  GET  endpoints: autocomplete, currencies, airport lookup");
    println!("  POST endpoints: search (w/ and w/o bgProgramResponse)");
    if solution_set.is_some() {
        println!("  POST endpoints: summarize, booking details");
        println!("  Search returned results — full pipeline working!");
    } else {
        println!("  Search did NOT return results — need to investigate bgProgramResponse");
    }

    Ok(())
}
