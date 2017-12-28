extern crate reqwest;
extern crate serde_json;

use self::serde_json::Value;

const ENDPOINT: &str = "https://newton.now.sh";
pub type DataResult = Result<String, reqwest::Error>;

pub fn simplify(expression: &str) -> DataResult {
    let url = format!("{}/simplify/{}", ENDPOINT, expression);
    fetch_result(&url)
}

fn fetch_result(url: &str) -> DataResult {
    let mut response = reqwest::get(url)?;
    let data: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    Ok(String::from(format!("{}",data["result"])))
}