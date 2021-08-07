pub async fn search(search_keyword: &str, key: &str) -> Result<String, reqwest::Error> {
    let url = format!(
        "{}{}{}{}",
        "https://www.googleapis.com/youtube/v3/search?&part=snippet&maxResults=50&type=video&q=\"",
        search_keyword.clone(),
        "\"&key=",
        &key[1..key.chars().count() - 1]
    );
    let res = reqwest::get(&url).await?;

    let body = res.text().await?;

    Ok(body)
}
