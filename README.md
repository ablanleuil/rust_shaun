# rust_shaun

Rust implementation of the SHAUN notation language.

To use it, add this dependency to your `Cargo.toml` file:

```
    [dependencies]
    shaun = { git = "https://git.studios-lalla.fr/ablanleuil/rust_shaun" }
```

```Rust
extern crate shaun;

use shaun::Shaun;

fn main()
{
    let mut sn : Shaun = shaun::parse_str("{ name: \"Jar Jar\", age: 15 y, amIRight: true }");

    if sn.is_object() {
        let mut map = sn.to_mut_map().unwrap();
        println!("{:?}", map.entry(String::from("name")))
    }
}
```
