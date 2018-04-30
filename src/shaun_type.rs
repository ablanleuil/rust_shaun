use std::collections::HashMap;
use std::convert::From;
use std::convert::TryInto;
use std::clone::Clone;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Shaun {
    Null(),
    Number(f64,String),
    String(String),
    Bool(bool),
    List(Vec<Shaun>),
    Object(HashMap<String,Shaun>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ShaunError {
    NotANumber(),
    NotAString(),
    NotABool(),
    NotAList(),
    NotAnObject(),
    OutOfBound(),
    AttributeNotFound(),
}

impl Shaun {
    pub fn is_object(&self) -> bool {
        match self {
            &Shaun::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            &Shaun::List(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            &Shaun::Null() => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            &Shaun::Number(_,_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            &Shaun::String(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            &Shaun::Bool(_) => true,
            _ => false,
        }
    }

    pub fn to_vec(&self) -> Result<&Vec<Shaun>, ShaunError> {
        match self {
            &Shaun::List(ref v) => Ok(v),
            _ => Err(ShaunError::NotAList()),
        }
    }

    pub fn to_map(&self) -> Result<&HashMap<String, Shaun>, ShaunError> {
        match self {
            &Shaun::Object(ref o) => Ok(o),
            _ => Err(ShaunError::NotAnObject()),
        }
    }

    pub fn to_mut_vec(&mut self) -> Result<&mut Vec<Shaun>, ShaunError> {
        match self {
            &mut Shaun::List(ref mut v) => Ok(v),
            _ => Err(ShaunError::NotAList()),
        }
    }

    pub fn to_mut_map(&mut self) -> Result<&mut HashMap<String, Shaun>, ShaunError> {
        match self {
            &mut Shaun::Object(ref mut o) => Ok(o),
            _ => Err(ShaunError::NotAnObject()),
        }
    }
}

impl TryInto<f64> for Shaun {
    type Error = ShaunError;
    fn try_into(self) -> Result<f64, ShaunError> {
        match self {
            Shaun::Number(x,_) => Ok(x),
            _ => Err(ShaunError::NotANumber()),
        }
    }
}

impl TryInto<String> for Shaun {
    type Error = ShaunError;
    fn try_into(self) -> Result<String, ShaunError> {
        match self {
            Shaun::String(s) => Ok(s),
            _ => Err(ShaunError::NotAString()),
        }
    }
}

impl TryInto<bool> for Shaun {
    type Error = ShaunError;
    fn try_into(self) -> Result<bool, ShaunError> {
        match self {
            Shaun::Bool(s) => Ok(s),
            _ => Err(ShaunError::NotABool()),
        }
    }
}

impl TryInto<Vec<Shaun>> for Shaun {
    type Error = ShaunError;
    fn try_into(self) -> Result<Vec<Shaun>, ShaunError> {
        match self {
            Shaun::List(v) => Ok(v),
            _ => Err(ShaunError::NotAList()),
        }
    }
}

impl TryInto<HashMap<String, Shaun>> for Shaun {
    type Error = ShaunError;
    fn try_into(self) -> Result<HashMap<String, Shaun>, ShaunError> {
        match self {
            Shaun::Object(o) => Ok(o),
            _ => Err(ShaunError::NotAnObject()),
        }
    }
}

impl<'a, T : Clone> From<&'a T> for Shaun where Shaun : From<T> {
    fn from(x:&'a T) -> Self {
        Shaun::from(x.clone())
    }
}

impl From<f64> for Shaun {
    fn from(x:f64) -> Self {
        Shaun::Number(x, String::new())
    }
}

impl From<String> for Shaun {
    fn from(x:String) -> Self {
        Shaun::String(x)
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
