use reqwest::Client;
use serde_json::Value;
use dotenv::dotenv;
use std::env;

pub struct TerminusDB {
    client: Client,
    url: String,
}

impl TerminusDB {
    pub fn new(url: &str) -> Self {
        TerminusDB {
            client: Client::new(),
            url: url.to_string(),
        }
    }

    pub async fn query(&self) -> Result<String, reqwest::Error> {
        dotenv().ok();
        let api_token = env::var("TERMINUS_TOKEN").expect("TERMINUS_TOKEN must be set");
        let team_name = env::var("TERMINUS_TEAM").expect("TERMINUS_TEAM must be set");
        let url = format!("{}/api/document/{}/family", self.url, team_name);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("API_TOKEN", api_token.parse().unwrap());
        let res: reqwest::Response = self.client.get(&url)
            .headers(headers)
            // .json(&json!({ "query": query }))
            .send()
            .await?;
        let body = res.text().await?;
        Ok(body)
    }
}

#[tokio::main]
pub async fn query() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let team_name = env::var("TERMINUS_TEAM").expect("TERMINUS_TEAM must be set");
    let db = TerminusDB::new(format!("https://cloud.terminusdb.com/{}", team_name).as_str());
    // let query = r#"{"@type": "woql:Select","woql:query_result": {"@type": "woql:Variable","woql:variable_name": "X"}}}"#;
    let resp = db.query().await?;
    let mut lines = resp.lines();
    while let Some(line) = lines.next() {
        let json: Value = serde_json::from_str(&line)?;
        println!("{}", json["name"]);
    }
    Ok(())
}