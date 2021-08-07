pub async fn search(search_keyword: &str, key: &str) -> Result<String, reqwest::Error> {
    //let client = reqwest::Client::new();
    //let search_keyword = "ben awad";
    let url = format!(
        "{}{}{}{}",
        "https://www.googleapis.com/youtube/v3/search?&part=snippet&maxResults=50&type=video&q=\"",
        search_keyword.clone(),
        "\"&key=",
        &key[1..key.chars().count() - 1]
    );
    let res = reqwest::get(&url).await?;

    // Move and borrow value of `res`
    let body = res.text().await?;

    Ok(body)
}
