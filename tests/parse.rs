extern crate shaun;

#[test]
fn parse_string() {
    shaun::parse_str("lol");
    assert_eq!(1, 1)
}
