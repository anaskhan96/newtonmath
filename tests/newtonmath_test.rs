extern crate newtonmath as newton;

#[test]
fn simplify() {
    assert_eq!("8", newton::simplify("2^2+2(2)").unwrap());
}

#[test]
fn factor() {
    assert_eq!("x (x + 2)", newton::factor("x^2+2x").unwrap());
}

#[test]
fn derive() {
    assert_eq!("2 x + 2", newton::derive("x^2+2x").unwrap());
}

#[test]
fn integrate() {
    assert_eq!("1/3 x^3 + x^2 + C", newton::integrate("x^2+2x").unwrap());
}

#[test]
fn find_zeroes() {
    assert_eq!("[-2, 0]", newton::find_zeroes("x^2+2x").unwrap());
}

#[test]
fn find_tangent() {
    assert_eq!("12 x + -16", newton::find_tangent("2lx^3").unwrap());
}

#[test]
fn area_under_curve() {
    assert_eq!("60", newton::area_under_curve("2:4lx^3").unwrap());
}

#[test]
fn cosine() {
    assert_eq!("-1", newton::cosine("pi").unwrap());
}

#[test]
fn sine() {
    assert_eq!("0", newton::sine("0").unwrap());
}

#[test]
fn tangent() {
    assert_eq!("0", newton::tangent("0").unwrap());
}

#[test]
fn inverse_cosine() {
    assert_eq!("0", newton::inverse_cosine("1").unwrap());
}

#[test]
fn inverse_sine() {
    assert_eq!("0", newton::inverse_sine("0").unwrap());
}

#[test]
fn inverse_tangent() {
    assert_eq!("0", newton::inverse_tangent("0").unwrap());
}

#[test]
fn absolute_value() {
    assert_eq!("1", newton::absolute_value("-1").unwrap());
}

#[test]
fn logarithm() {
    assert_eq!("3", newton::logarithm("2l8").unwrap());
}
