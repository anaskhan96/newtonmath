extern crate reqwest;
extern crate serde_json;

use self::serde_json::Value;

const ENDPOINT: &str = "https://newton.now.sh";
pub type DataResult = Result<String, reqwest::Error>;

pub fn simplify(expression: &str) -> DataResult {
    let url = format!("{}/simplify/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn factor(expression: &str) -> DataResult {
    let url = format!("{}/factor/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn derive(expression: &str) -> DataResult {
    let url = format!("{}/derive/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn integrate(expression: &str) -> DataResult {
    let url = format!("{}/integrate/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn find_zeroes(expression: &str) -> DataResult {
    let url = format!("{}/zeroes/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn find_tangent(expression: &str) -> DataResult {
    let url = format!("{}/tangent/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn area_under_curve(expression: &str) -> DataResult {
    let url = format!("{}/area/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn cosine(expression: &str) -> DataResult {
    let url = format!("{}/cos/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn sine(expression: &str) -> DataResult {
    let url = format!("{}/sin/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn tangent(expression: &str) -> DataResult {
    let url = format!("{}/tan/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn inverse_cosine(expression: &str) -> DataResult {
    let url = format!("{}/arccos/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn inverse_sine(expression: &str) -> DataResult {
    let url = format!("{}/arcsin/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn inverse_tangent(expression: &str) -> DataResult {
    let url = format!("{}/arctan/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn absolute_value(expression: &str) -> DataResult {
    let url = format!("{}/abs/{}", ENDPOINT, expression);
    fetch_result(&url)
}

pub fn logarithm(expression: &str) -> DataResult {
    let url = format!("{}/log/{}", ENDPOINT, expression);
    fetch_result(&url)
}

fn fetch_result(url: &str) -> DataResult {
    let mut response = reqwest::get(url)?;
    let data: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    Ok(String::from(format!("{}",data["result"])))
}