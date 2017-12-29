extern crate reqwest;
extern crate serde_json;

use self::serde_json::Value;

const ENDPOINT: &str = "https://newton.now.sh";
pub type StringResult = Result<String, reqwest::Error>;
pub type VectorResult = Result<Vec<i64>, reqwest::Error>;

/// /simplify endpoint
pub fn simplify(expression: &str) -> StringResult {
    let url = format!("{}/simplify/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /factor endpoint
pub fn factor(expression: &str) -> StringResult {
    let url = format!("{}/factor/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /derive endpoint
pub fn derive(expression: &str) -> StringResult {
    let url = format!("{}/derive/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /integrate endpoint
pub fn integrate(expression: &str) -> StringResult {
    let url = format!("{}/integrate/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /zeroes endpoint
pub fn find_zeroes(expression: &str) -> VectorResult {
    let url = format!("{}/zeroes/{}", ENDPOINT, expression);
    let mut response = reqwest::get(&url)?;
    let data: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    let mut val = Vec::new();
    for i in data["result"].as_array().unwrap() {
        val.push(i.as_i64().unwrap());
    }
    Ok(val)
}

/// /tangent endpoint
pub fn find_tangent(expression: &str) -> StringResult {
    let url = format!("{}/tangent/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /area endpoint
pub fn area_under_curve(expression: &str) -> StringResult {
    let url = format!("{}/area/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /cos endpoint
pub fn cosine(expression: &str) -> StringResult {
    let url = format!("{}/cos/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /sin endpoint
pub fn sine(expression: &str) -> StringResult {
    let url = format!("{}/sin/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /tan endpoint
pub fn tangent(expression: &str) -> StringResult {
    let url = format!("{}/tan/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /arccos endpoint
pub fn inverse_cosine(expression: &str) -> StringResult {
    let url = format!("{}/arccos/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /arcsin endpoint
pub fn inverse_sine(expression: &str) -> StringResult {
    let url = format!("{}/arcsin/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /arctan endpoint
pub fn inverse_tangent(expression: &str) -> StringResult {
    let url = format!("{}/arctan/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /abs endpoint
pub fn absolute_value(expression: &str) -> StringResult {
    let url = format!("{}/abs/{}", ENDPOINT, expression);
    fetch_result(&url)
}

/// /log endpoint
pub fn logarithm(expression: &str) -> StringResult {
    let url = format!("{}/log/{}", ENDPOINT, expression);
    fetch_result(&url)
}

fn fetch_result(url: &str) -> StringResult {
    let mut response = reqwest::get(url)?;
    let data: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    Ok(String::from(data["result"].as_str().unwrap()))
}
