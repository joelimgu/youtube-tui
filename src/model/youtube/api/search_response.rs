#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    kind: String,
    etag: String,
    nextPageToken: String,
    regionCode: String,
    pageInfo: PageInfo,
    pub(crate) items: Vec<SearchResult>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PageInfo {
    totalResults: u32,
    resultsPerPage: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    kind: String,
    etag: String,
    id: Id,
    pub(crate) snippet: Snippet,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    kind: String,
    videoId: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Snippet {
    publishedAt: String,
    channelId: String,
    pub(crate) title: String,
    description: String,
    thumbnails: Thumbnails,
    channelTitle: String,
    liveBroadcastContent: String,
    publishTime: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnails {
    default: Thumbnail,
    medium: Thumbnail,
    high: Thumbnail,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    url: String,
    width: u32,
    height: u32,
}
