use ita_matrix::ItaClient;
use ita_matrix::model::search::SearchRequest;
use ita_matrix::model::summarize::SummarizeRequest;

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .with_test_writer()
        .try_init();
}

fn client() -> ItaClient {
    ItaClient::new().expect("failed to create client")
}

#[tokio::test]
async fn autocomplete_phx() {
    init_tracing();
    let locations = client().autocomplete("PHX", 10).await.unwrap();
    assert!(!locations.is_empty());
    assert_eq!(locations[0].code, "PHX");
    println!("{}", serde_json::to_string_pretty(&locations).unwrap());
}

#[tokio::test]
async fn currencies() {
    init_tracing();
    let currencies = client().currencies().await.unwrap();
    assert!(currencies.len() > 50);
    assert!(currencies.iter().any(|c| c.code == "USD"));
    println!("{} currencies", currencies.len());
}

#[tokio::test]
async fn airport_lookup() {
    init_tracing();
    let airport = client().lookup_airport("PHX").await.unwrap();
    assert_eq!(airport.code, "PHX");
    assert_eq!(airport.city_code.as_deref(), Some("PHX"));
    println!("{}", serde_json::to_string_pretty(&airport).unwrap());
}

#[tokio::test]
async fn search_one_way() {
    init_tracing();
    let req = SearchRequest::builder()
        .origins(&["PHX"])
        .destinations(&["DTW"])
        .date("2026-02-15")
        .build();

    let resp = client().search(&req).await.unwrap();

    assert!(
        resp.error.is_none(),
        "search returned error: {:?}",
        resp.error
    );
    assert!(resp.solution_set.is_some(), "missing solutionSet");
    assert!(resp.session.is_some(), "missing session");

    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}

#[tokio::test]
async fn search_round_trip() {
    init_tracing();
    let req = SearchRequest::builder()
        .round_trip(&["PHX"], &["DTW"], "2026-02-15", "2026-02-22")
        .build();

    let resp = client().search(&req).await.unwrap();

    assert!(
        resp.error.is_none(),
        "search returned error: {:?}",
        resp.error
    );
    assert!(resp.solution_set.is_some(), "missing solutionSet");
    assert!(resp.session.is_some(), "missing session");
    assert!(resp.solution_list.is_some(), "missing solutionList");

    let sol_list = resp.solution_list.as_ref().unwrap();
    assert!(
        !sol_list.solutions.is_empty(),
        "no solutions in round-trip search"
    );

    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}

#[tokio::test]
async fn search_calendar() {
    init_tracing();
    let req = SearchRequest::calendar(&["PHX"], &["DTW"], "2026-02-01", "2026-02-28");

    let resp = client().search(&req).await.unwrap();

    assert!(
        resp.error.is_none(),
        "calendar search returned error: {:?}",
        resp.error
    );
    assert!(resp.session.is_some(), "missing session");

    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}

#[tokio::test]
async fn search_then_summarize() {
    init_tracing();
    let req = SearchRequest::builder()
        .origins(&["PHX"])
        .destinations(&["DTW"])
        .date("2026-02-15")
        .build();

    let search_resp = client().search(&req).await.unwrap();
    assert!(search_resp.error.is_none());

    let ss = search_resp
        .solution_set
        .as_ref()
        .expect("missing solutionSet");
    let sess = search_resp.session.as_ref().expect("missing session");

    let summarize_req = SummarizeRequest::new(ss, sess);
    let summarize_resp = client().summarize(&summarize_req).await.unwrap();

    assert!(
        summarize_resp.error.is_none(),
        "summarize returned error: {:?}",
        summarize_resp.error
    );
    println!("{}", serde_json::to_string_pretty(&summarize_resp).unwrap());
}

#[tokio::test]
async fn search_then_booking_details() {
    init_tracing();
    let req = SearchRequest::builder()
        .origins(&["PHX"])
        .destinations(&["DTW"])
        .date("2026-02-15")
        .build();

    let search_resp = client().search(&req).await.unwrap();
    assert!(search_resp.error.is_none());

    let ss = search_resp
        .solution_set
        .as_ref()
        .expect("missing solutionSet");
    let sess = search_resp.session.as_ref().expect("missing session");
    let sol_list = search_resp
        .solution_list
        .as_ref()
        .expect("missing solutionList");
    let first_solution = &sol_list.solutions[0];

    let full_solution_id = format!("{}/{}", ss, first_solution.id);
    let details_req = SummarizeRequest::booking_details(ss, sess, &full_solution_id);
    let details_resp = client().summarize(&details_req).await.unwrap();

    assert!(
        details_resp.error.is_none(),
        "booking details returned error: {:?}",
        details_resp.error
    );

    assert!(
        details_resp.booking_details.is_some(),
        "missing typed bookingDetails"
    );
    let bd = details_resp.booking_details.as_ref().unwrap();
    assert!(
        bd.itinerary.is_some(),
        "missing itinerary in bookingDetails"
    );

    println!("{}", serde_json::to_string_pretty(&details_resp).unwrap());
}

#[tokio::test]
async fn search_then_fare_rules() {
    init_tracing();
    let req = SearchRequest::builder()
        .origins(&["PHX"])
        .destinations(&["DTW"])
        .date("2026-02-15")
        .build();

    let search_resp = client().search(&req).await.unwrap();
    assert!(search_resp.error.is_none());

    let ss = search_resp
        .solution_set
        .as_ref()
        .expect("missing solutionSet");
    let sess = search_resp.session.as_ref().expect("missing session");
    let sol_list = search_resp
        .solution_list
        .as_ref()
        .expect("missing solutionList");
    let first_solution = &sol_list.solutions[0];

    let solution_id = format!("{}/{}", ss, first_solution.id);
    let fare_req = SummarizeRequest::fare_rules(ss, sess, &solution_id, 0, 0);
    let fare_resp = client().summarize(&fare_req).await.unwrap();

    assert!(
        fare_resp.error.is_none(),
        "fare rules returned error: {:?}",
        fare_resp.error
    );
    assert!(
        fare_resp.fare_rules.is_some(),
        "missing fareRules in response"
    );
    println!("{}", serde_json::to_string_pretty(&fare_resp).unwrap());
}
