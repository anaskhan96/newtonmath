# newtonmath
[![Build Status](https://travis-ci.org/anaskhan96/newtonmath.svg?branch=master)](https://travis-ci.org/anaskhan96/newtonmath)
[![Crates.io](https://img.shields.io/crates/v/rustc-serialize.svg)](https://crates.io/crates/newtonmath)

`newtonmath` is a Rust wrapper for the [Newton API](https://github.com/aunyks/newton-api), *a really micro micro-service for advanced math*.

Functions implemented:
```rust
fn simplify(exp: &str) -> StringResult // /simplify endpoint
fn factor(exp: &str) -> StringResult // /factor endpoint
fn derive(exp: &str) -> StringResult // /derive endpoint
fn integrate(exp: &str) -> StringResult // /integrate endpoint
fn find_zeroes(exp: &str) -> VectorResult // /zeroes endpoint
fn find_tangent(exp: &str) -> StringResult // /tangent endpoint
fn area_under_curve(exp: &str) -> StringResult // /area endpoint
fn cosine(exp: &str) -> StringResult // /cos endpoint
fn sine(exp: &str) -> StringResult // /sin endpoint
fn tangent(exp: &str) -> StringResult // /tan endpoint
fn inverse_cosine(exp: &str) -> StringResult // /arccos endpoint
fn inverse_sine(exp: &str) -> StringResult // /arcsin endpoint
fn inverse_tangent(exp: &str) -> StringResult // /arctan endpoint
fn absolute_value(exp: &str) -> StringResult // /abs endpoint
fn logarithm(exp: &str) -> StringResult // /log endpoint
```
The `StringResult` returned is of type `Result<String, reqwest::Error>` whereas the `VectorResult` returned is of type `Result<Vec<i64>, reqwest::Error>`.

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