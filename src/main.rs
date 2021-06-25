// mod tui;
// extern crate reqwest;
// use std::env;
// use std::fs;
// use std::path::PathBuf;
// use futures::TryFutureExt;
// use futures::executor;
// use youtube_dl::YoutubeDl;

// async fn req(key: &str) -> Result<String, reqwest::Error> {
//     //let client = reqwest::Client::new();
//     let search_keyword = "ben awad";
//     let url = format!("{}{}{}{}", "https://www.googleapis.com/youtube/v3/search?&maxResults=25&q=\"",  search_keyword.clone(), "\"&key=", &key[1..key.chars().count() - 1]);
//     let res = reqwest::get(&url).await?;

//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());

//     // Move and borrow value of `res`
//     let body = res.text().await?;
//     println!("Body:n{}", body);
//     println!("url used:{}", url);

//     Ok(body)
// }


// async fn download(){

//     let url = "https://www.youtube.com/watch?v=Edx9D2yaOGs&ab_channel=CollegeHumor";
//     let path_to_video = rustube::download_best_quality(url).await;
    

//     print!("path:{:?}", path_to_video);
//     println!("HERE")
// }

//#[tokio::main(flavor = "current_thread")]
// fn main() {
//     /*
//     let contents = fs::read_to_string("/home/joel/Documents/Code/youtube-tui/src/config.json")
//         .expect("Something went wrong reading the file");
//     println!("With text:\n{}", contents);
    
//     let json: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");
//     println!("JSON:{}", json["key"]);
//     let res: serde_json::Value = serde_json::from_str(&req(&json["key"].to_string()).unwrap())
//         .expect("JSON was not well-formatted");
    
    
//     let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
//         .socket_timeout("15")
//         .run()
//         .unwrap();

//     let output = match output {
//         youtube_dl::YoutubeDlOutput::Playlist(_) => {Err("playlists not supported yet")},
//         youtube_dl::YoutubeDlOutput::SingleVideo(res) => {Ok(*res)}
//     };

//     println!("test:{:?}", output.unwrap().downloader_options);
//     */
//     // let handle = tokio::runtime::Handle::current();
//     // handle.enter();

//     //download().await;
//     let url = "https://youtu.be/nv2wQvn6Wxc";
//     let path_to_video = rustube::blocking::download_best_quality(url).unwrap();
//     println!("{:?}", path_to_video)
// }

// working!!
// #[rustube::tokio::main]
// async fn main() {
//     let url = "https://www.youtube.com/watch?v=Edx9D2yaOGs&ab_channel=CollegeHumor";
//     println!("downloaded video to {:?}", rustube::download_best_quality(&url).await.unwrap());
// }


mod tui;

fn main(){
    let _y = tui::render();
}