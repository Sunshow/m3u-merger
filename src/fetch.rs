pub(crate) async fn fetch_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(resp)
}

