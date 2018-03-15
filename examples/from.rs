extern crate shaun;

use shaun::Shaun;
use shaun::Shaun::{Bool, List};

fn main() {
    let mut val = Shaun::from(true);

    match val {
        Bool(tf) => if tf { println!("val is true.") } else { println!("val is false.") },
        _ => println!("val is not a Bool.")
    }

    val = Shaun::from(vec![10.0,20.0]);

    match val {
        List(vec) => for x in vec { println!("{:?}", x) },
        _ => println!("val is not a List.")
    }
}
