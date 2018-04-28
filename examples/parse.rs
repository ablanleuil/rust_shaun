extern crate shaun;

fn main() {
    let val = shaun::parse_str("{ hello : \"world\", test_number : 10.5 ms }");
    println!("{:?}", val)
}
