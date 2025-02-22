mod hacker_news;
use tokio::fs;
use serde_json;

#[tokio::main]
async fn main() {
    match hacker_news::get_hacker_news_list().await {
        Ok(data) => {
            let json_string = serde_json::to_string_pretty(&data).expect("Failed to serialize JSON");

            if let Err(err) = fs::write("data.json", json_string).await {
                eprintln!("Failed to save data.json: {:?}", err);
            } else {
                println!("Data saved to data.json");
            }
        }
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
