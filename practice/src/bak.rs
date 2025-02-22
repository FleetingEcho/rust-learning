// use reqwest;
// use serde_json::{json, Value};
// use std::sync::{
//    atomic::{AtomicUsize, Ordering},
//    Arc,
// };
// use std::collections::HashMap;
// use tokio::task::spawn;
// use futures::stream::{FuturesUnordered, StreamExt};


// const BASE_URL: &str = "http://aaa.com/v1";
// const SUMMARY_API: &str = "/srvupd/summary?limit={}&types=rhsp";
// const DETAIL_API: &str = "/srvupd/detail/rhsp/{}";
// const CONTENT_API: &str = "/fgd/lookup/ency?source=rhsp&id={}";


// #[derive(Clone)]
// struct IDGenerator {
//    counter: Arc<AtomicUsize>,
// }

// impl IDGenerator {
//    fn new() -> Self {
//        IDGenerator {
//            counter: Arc::new(AtomicUsize::new(0)),
//        }
//    }
//    fn next(&self, prefix: &str) -> String {
//        let id = self.counter.fetch_add(1, Ordering::Relaxed);
//        format!("{}-{}", prefix, id)
//    }
// }

// async fn fetch_json(url: &str) -> Result<Value, reqwest::Error> {
//    reqwest::get(url).await?.json().await
// }

// async fn get_latest_packages(limit: usize) -> Result<Vec<Value>, reqwest::Error> {
//    let url = format!("{}{}", BASE_URL, SUMMARY_API.replace("{}", &limit.to_string()));
//    fetch_json(&url)
//        .await
//        .map(|v| v.as_array().cloned().unwrap_or_default())
// }


// async fn get_package_details(version: &str) -> Result<Vec<Value>, reqwest::Error> {
//    let url = format!("{}{}", BASE_URL, DETAIL_API.replace("{}", version));
//    fetch_json(&url)
//        .await
//        .map(|mut v| v.as_array_mut().map(|v| std::mem::take(v)).unwrap_or_default())
// }

// async fn get_content_details(content_id: &str) -> Result<Value, reqwest::Error> {
//    let url = format!("{}{}", BASE_URL, CONTENT_API.replace("{}", content_id));
//    fetch_json(&url).await
// }

// async fn process_details(details: Vec<Value>, parent_id: String, id_gen: Arc<IDGenerator>) -> Vec<Value> {
//    let mut tasks = Vec::new();
//    let mut items_map = HashMap::new();

//    for mut item in details {
//        let item_id = id_gen.next(&format!("{}-group", parent_id));
//        item["_id"] = json!(item_id.clone());
//        items_map.insert(item_id.clone(), item);
//    }

//    for (item_id, item) in items_map.clone().into_iter() {
//        let content_id = item["ID"].as_str().map(str::to_string)
//            .or_else(|| item["ID"].as_i64().map(|id| id.to_string()))
//            .unwrap_or_default();

//        if !content_id.is_empty() {
//            let id_gen = id_gen.clone();
//            let item_id_clone = item_id.clone();

//            tasks.push(spawn(async move {
//                match get_content_details(&content_id).await {
//                    Ok(mut detail_data) => {
//                        let detail_id = id_gen.next(&format!("{}-detail", item_id_clone));
//                        detail_data["_id"] = json!(detail_id);
//                        Some((item_id_clone, detail_data))
//                    }
//                    Err(_) => None,
//                }
//            }));
//        }
//    }

//    let content_results: Vec<_> = futures::future::join_all(tasks)
//        .await
//        .into_iter()
//        .filter_map(|res| res.ok())
//        .flatten()
//        .collect();

//    for (item_id, detail) in content_results {
//        if let Some(item) = items_map.get_mut(&item_id) {
//            item["_details"] = json!(detail);
//        }
//    }

//    items_map.into_values().collect()
// }


// async fn process_package(
//    package: Value,
//    id_gen: Arc<IDGenerator>,
// ) -> Result<Value, String> {
//    let mut package_data = package.clone();
//    let package_id = id_gen.next("entry");
//    package_data["_id"] = json!(package_id.clone());

//    let version = match package["Version"].as_str() {
//        Some(v) => v,
//        None => return Err("Missing package version".to_string()),
//    };

//    let details = match get_package_details(version).await {
//        Ok(d) => d,
//        Err(e) => return Err(format!("Error fetching details for {}: {}", version, e)),
//    };

//    let processed_details = process_details(details, package_id, id_gen.clone()).await;
//    package_data["_content"] = json!(processed_details);

//    Ok(package_data)
// }

// async fn get_soc_automation_list(limit: usize) -> Value {
//    let packages = match get_latest_packages(limit).await {
//        Ok(pkgs) => pkgs,
//        Err(e) => return json!({ "status": "error", "errors": [format!("{}", e)] }),
//    };

//    let id_gen = Arc::new(IDGenerator::new());
//    let mut tasks = FuturesUnordered::new();

//    for package in packages {
//        let id_gen = id_gen.clone();

//        tasks.push(spawn(async move {
//            process_package(package, id_gen).await
//        }));
//    }

//    let mut processed_packages = Vec::new();
//    while let Some(result) = tasks.next().await {
//        if let Ok(Ok(pkg)) = result {
//            processed_packages.push(pkg);
//        }
//    }

//    json!({ "status": "success", "data": processed_packages })
// }

// #[tokio::main]
// async fn main() {
//    let limit = 20;
//    let start_time = std::time::Instant::now();
//    let result = get_soc_automation_list(limit).await;
//    println!("Total execution time: {:?}", start_time.elapsed());

//    match tokio::fs::write("rust-data.json", serde_json::to_string_pretty(&result).unwrap()).await {
//        Ok(_) => println!("Data saved successfully"),
//        Err(e) => eprintln!("Error writing JSON: {}", e),
//    }
// }