use ita_matrix::model::search::SearchRequest;
use ita_matrix::model::summarize::SummarizeRequest;
use ita_matrix::ItaClient;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let client = ItaClient::new()?;

    info!("--- autocomplete ---");
    let locations = client.autocomplete("PHX", 10).await?;
    println!("{}", serde_json::to_string_pretty(&locations)?);

    info!("--- currencies ---");
    let currencies = client.currencies().await?;
    println!("{} currencies loaded", currencies.len());

    info!("--- airport lookup ---");
    let airport = client.lookup_airport("PHX").await?;
    println!("{}", serde_json::to_string_pretty(&airport)?);

    info!("--- flight search ---");
    let search_req = SearchRequest::builder()
        .origins(&["PHX"])
        .destinations(&["DTW"])
        .date("2026-02-15")
        .build();

    let search_resp = client.search(&search_req).await?;
    println!("{}", serde_json::to_string_pretty(&search_resp)?);

    if let (Some(ss), Some(sess)) = (&search_resp.solution_set, &search_resp.session) {
        info!("--- summarize ---");
        let summarize_req = SummarizeRequest::new(ss, sess);
        let summarize_resp = client.summarize(&summarize_req).await?;
        println!("{}", serde_json::to_string_pretty(&summarize_resp)?);

        if let Some(ref sol_list) = search_resp.solution_list
            && let Some(first) = sol_list.solutions.first() {
                info!("--- booking details ---");
                let details_req =
                    SummarizeRequest::booking_details(ss, sess, &first.id);
                let details_resp = client.summarize(&details_req).await?;
                println!("{}", serde_json::to_string_pretty(&details_resp)?);
            }
    }

    Ok(())
}
