extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate failure;

use self::serde_json::Value;
use failure::Error;

const ENDPOINT: &str = "https://newton.now.sh";
pub type StringResult = Result<String, Error>;
pub type VectorResult = Result<Vec<i64>, Error>;

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
    let data: Value = serde_json::from_str(&response.text()?)?;
    let mut val = Vec::new();
    match data["result"].as_array() {
        None => Err(format_err!("Result not an array")),
        Some(arr) => {
            for i in arr {
                match i.as_i64() {
                    None => return Err(format_err!("Element in array not an integer")),
                    Some(ele) => val.push(ele),
                };
            }
            Ok(val)
        }
    }
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
    let data: Value = serde_json::from_str(&response.text()?)?;
    match data["result"].as_str() {
        None => Err(format_err!("Result not a string")),
        Some(val) => Ok(String::from(val)),
    }
}
