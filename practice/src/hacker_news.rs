use reqwest;
use anyhow::Result;
use serde_json::Value;
use tokio::task::JoinSet;

const HACKER_NEWS_TOP_STORIES: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";
const HACKER_NEWS_ITEM: &str = "https://hacker-news.firebaseio.com/v0/item/";

pub async fn get_hacker_news_list() -> Result<Vec<Value>> {
    let story_ids: Vec<u64> = reqwest::get(HACKER_NEWS_TOP_STORIES)
        .await?
        .json::<Vec<u64>>()
        .await?;

    // let top_10_ids = story_ids.into_iter().take(10);
    let top_10_ids = story_ids.get(..10).unwrap_or(&story_ids);


    let mut tasks = JoinSet::new();

    for id in top_10_ids {
        let url = format!("{HACKER_NEWS_ITEM}{id}.json");
        tasks.spawn(async move {
            reqwest::get(&url).await?.json::<Value>().await
        });
    }

    let mut stories = Vec::new();
    while let Some(result) = tasks.join_next().await {
        if let Ok(Ok(story)) = result {
            stories.push(story);
        }
    }
    Ok(stories)
}



/*
    let mut stories = Vec::new();
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(story)) => {
                stories.push(story);
            }
            Ok(Err(err)) => {
                eprintln!("Failed to fetch or parse JSON: {:?}", err);
            }
            Err(join_err) => {
                eprintln!("Task join error: {:?}", join_err);
            }
        }
    }
*/