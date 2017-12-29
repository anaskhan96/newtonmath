# newtonmath

`newtonmath` is a Rust wrapper for the [Newton API](https://github.com/aunyks/newton-api), *a really micro micro-service for advanced math*.

Functions implemented:
```rust
fn simplify(exp: &str) -> DataResult // /simplify endpoint
fn factor(exp: &str) -> DataResult // /factor endpoint
fn derive(exp: &str) -> DataResult // /derive endpoint
fn integrate(exp: &str) -> DataResult // /integrate endpoint
fn find_zeroes(exp: &str) -> DataResult // /zeroes endpoint
fn find_tangent(exp: &str) -> DataResult // /tangent endpoint
fn area_under_curve(exp: &str) -> DataResult // /area endpoint
fn cosine(exp: &str) -> DataResult // /cos endpoint
fn sine(exp: &str) -> DataResult // /sin endpoint
fn tangent(exp: &str) -> DataResult // /tan endpoint
fn inverse_cosine(exp: &str) -> DataResult // /arccos endpoint
fn inverse_sine(exp: &str) -> DataResult // /arcsin endpoint
fn inverse_tangent(exp: &str) -> DataResult // /arctan endpoint
fn absolute_value(exp: &str) -> DataResult // /abs endpoint
fn logarithm(exp: &str) -> DataResult // /log endpoint
```
The `DataResult` returnedis of type `Result<String, reqwest::Error>`.

### Setup

* Add `newtonmath` as a dependency in your `Cargo.toml`
```toml
[dependencies]
newtonmath = "0.2.2"
```

* Include it in your code
```rust
extern crate newtonmath as newton;
```

### Usage
```rust
fn main(){
    let res = newton::derive("x^2-1");
    match res {
        Ok(data) => println!("{}", data), // "2 x"
        Err(err) => println!("{:?}",err) // If an error is returned
    };
}
```