use std::collections::HashMap;
use std::convert::From;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Shaun {
    Null(),
    Number(f64,String),
    String(String),
    Bool(bool),
    List(Vec<Shaun>),
    Object(HashMap<String,Shaun>),
}

impl From<String> for Shaun {
    fn from(x:String) -> Self {
        Shaun::String(x)
    }
}

impl From<f64> for Shaun {
    fn from(x:f64) -> Self {
        Shaun::Number(x, String::new())
    }
}

impl From<bool> for Shaun {
    fn from(x:bool) -> Self {
        Shaun::Bool(x)
    }
}

impl<T> From<Vec<T>> for Shaun where Shaun : From<T> {
    fn from(x:Vec<T>) -> Self {
        Shaun::List(x.into_iter().map(Shaun::from).collect())
    }
}

impl<T> From<HashMap<String,T>> for Shaun where Shaun : From<T> {
    fn from(x:HashMap<String,T>) -> Self {
        Shaun::Object(x.into_iter().map(|(k,v)| { (k, Shaun::from(v)) }).collect())
    }
}

impl<'a> From<&'a str> for Shaun {
    fn from(x:&'a str) -> Self {
        Shaun::String(String::from(x))
    }
}
