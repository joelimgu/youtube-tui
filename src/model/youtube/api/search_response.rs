#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResponse {
    kind: String,
    etag: String,
    nextPageToken: String,
    regionCode: String,
    pageInfo: PageInfo,
    pub(crate) items: Vec<SearchResult>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageInfo {
    totalResults: u32,
    resultsPerPage: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    kind: String,
    etag: String,
    pub id: Id,
    pub(crate) snippet: Snippet,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Id {
    kind: String,
    pub videoId: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snippet {
    publishedAt: String,
    channelId: String,
    pub(crate) title: String,
    description: String,
    pub(crate) thumbnails: Thumbnails,
    channelTitle: String,
    liveBroadcastContent: String,
    publishTime: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thumbnails {
    pub(crate) default: Thumbnail,
    medium: Thumbnail,
    high: Thumbnail,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thumbnail {
    pub(crate) url: String,
    width: u32,
    height: u32,
}
