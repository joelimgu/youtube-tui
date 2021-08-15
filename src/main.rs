use std::fs;
mod model;

use model::youtube::api::search_response::SearchResponse;
extern crate jpeg_decoder as jpeg;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    /* get thumbnail */
    // let res = reqwest::get("https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg")
    //     .await
    //     .unwrap();
    //
    // let bytes: Bytes = res.bytes().await.unwrap();
    // let img: RgbImage = ImageReader::new(Cursor::new(&bytes))
    //     .with_guessed_format()
    //     .expect("can't guess the format")
    //     .decode()
    //     .expect("Could not decode the image")
    //     .to_rgb8();

    // video_player::print_img(img);
    /*
        let mut keyword = String::new();
        print!("Enter your search term: ");

        io::stdin()
            .read_line(&mut keyword)
            .expect("Sorry, input could not be read");
    */
    let contents = fs::read_to_string("/home/joel/Documents/Code/youtube-tui/src/config.json")
        .expect("Something went wrong reading the file");

    let config: serde_json::Value =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    let result = model::youtube::api::requests::search_videos::search(
        "hermitcraft",
        &config["key"].to_string(),
    )
    .await;

    let search_result: SearchResponse = serde_json::from_str(&result.expect("http request failed"))
        .expect("failed parsing the json");

    let mut screen = model::tui::screen::Screen::new(search_result).await;
    let _ = screen.render().await;
}

// async fn download(){

//     let url = "https://www.youtube.com/watch?v=Edx9D2yaOGs&ab_channel=CollegeHumor";
//     let path_to_video = rustube::download_best_quality(url).await;

//     print!("path:{:?}", path_to_video);
//     println!("HERE")
// }

//#[tokio::main(flavor = "current_thread")]
// fn main() {
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
