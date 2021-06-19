
extern crate reqwest;
use std::env;
use std::fs;
use youtube_dl::YoutubeDl;


#[tokio::main]
async fn req(key: &str) -> Result<String, reqwest::Error> {
    //let client = reqwest::Client::new();
    let search_keyword = "ben awad";
    let url = format!("{}{}{}{}", "https://www.googleapis.com/youtube/v3/search?&maxResults=25&q=\"",  search_keyword.clone(), "\"&key=", &key[1..key.chars().count() - 1]);
    let res = reqwest::get(&url).await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    // Move and borrow value of `res`
    let body = res.text().await?;
    println!("Body:n{}", body);
    println!("url used:{}", url);

    Ok(body)
}

fn main(){
    let contents = fs::read_to_string("/home/joel/Documents/Code/youtube-tui/src/config.json")
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
    
    let json: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    println!("JSON:{}", json["key"]);
    let res: serde_json::Value = serde_json::from_str(&req(&json["key"].to_string()).unwrap())
        .expect("JSON was not well-formatted");
    
    let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
        .socket_timeout("15")
        .run()
        .unwrap();
    println!("codec:{}", output.to_single_video());
}