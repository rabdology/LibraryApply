use serde::{Deserialize, Serialize};
use reqwest::Error;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Library {
    systemid: String,
    systemname: String,
    formal: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = "your_api_key";
    let isbn = "9784043636068"; // 任意のISBN

    let url = format!("https://api.calil.jp/library?appkey={}&format=json&isbn={}", api_key, isbn);

    let response: Vec<Library> = reqwest::get(&url).await?.json().await?;

    println!("図書館一覧:");
    for library in &response {
        println!(
            "systemid: {}, systemname: {}, formal: {}",
            library.systemid, library.systemname, library.formal
        );
    }

    Ok(())
}
