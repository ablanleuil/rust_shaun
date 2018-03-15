extern crate shaun;

use shaun::Shaun;
use shaun::Shaun::*;

#[test]
fn from_float() {
    assert_eq!(Shaun::Number(10.0, std::string::String::new()), Shaun::from(10.0))
}
