extern crate shaun;

use shaun::Shaun;

fn main() {
    let mut sn = shaun::parse_str("{ hello : \"world\", test_number : 10.5 ms }");

    match sn.to_mut_map() {
        Ok(v) => {
            println!("{:?}", v.entry(String::from("hello")).or_insert(Shaun::Null));
            println!("{:?}", v.entry(String::from("hi")).or_insert(Shaun::from("how are you ?")));
        },
        Err(e) => println!("{:?}", e),
    }

    println!("{:?}", sn.to_mut_map().unwrap().entry(String::from("hi")).or_insert(Shaun::Null()))
}
