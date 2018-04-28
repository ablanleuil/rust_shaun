extern crate shaun;

#[test]
fn parse_functions_equal() {
    let v1 = shaun::parse_str("hello : true");
    let v2 = shaun::parse_string("hello : true".to_string());
    assert_eq!(v1, v2)
}

#[test]
fn parse_root_bracket() {
    let v1 = shaun::parse_str("hello : true");
    let v2 = shaun::parse_str("{ hello : true }");
    assert_eq!(v1, v2)
}
