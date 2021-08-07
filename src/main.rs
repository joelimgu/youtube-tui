// mod tui;
// extern crate reqwest;
// use std::env;
// use std::fs;
// use std::path::PathBuf;
// use futures::TryFutureExt;
// use futures::executor;
// use youtube_dl::YoutubeDl;

use std::{fs, io};

mod model;

use model::youtube::api::search_response::SearchResponse;

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
//     let url = "https://www.youtube.com/watch?v=Edx9 D2yaOGs&ab_channel=CollegeHumor";
//     println!("downloaded video to {:?}", rustube::download_best_quality(&url).await.unwrap());
// }

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut keyword = String::new();
    print!("Enter your search term: ");

    io::stdin()
        .read_line(&mut keyword)
        .expect("Sorry, input could not be read");

    let contents = fs::read_to_string("/home/joel/Documents/Code/youtube-tui/src/config.json")
        .expect("Something went wrong reading the file");

    let config: serde_json::Value =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    let result = model::youtube::api::requests::search_videos::search(
        keyword.as_str(),
        &config["key"].to_string(),
    )
    .await;

    let search_result: SearchResponse = serde_json::from_str(&result.expect("http request failed"))
        .expect("failed parsing the json");

    let mut screen = model::tui::screen::Screen::new(search_result);
    let _ = screen.render();
    // let _y = model::tui::screen::Screen::render(search_result);
}
