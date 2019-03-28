use visitor::Visitor;

use std::collections::HashMap;
use std::convert::From;
use std::convert::TryInto;
use std::clone::Clone;
use std::ops::Index;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Shaun {
    Null,
    Number(f64,String),
    String(String),
    Bool(bool),
    List(Vec<Shaun>),
    Object(HashMap<String,Shaun>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ShaunError {
    NotANumber,
    NotAString,
    NotABool,
    NotAList,
    NotAnObject,
    OutOfBound,
    AttributeNotFound,
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
            &Shaun::Null => true,
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
            _ => Err(ShaunError::NotAList),
        }
    }

    pub fn to_map(&self) -> Result<&HashMap<String, Shaun>, ShaunError> {
        match self {
            &Shaun::Object(ref o) => Ok(o),
            _ => Err(ShaunError::NotAnObject),
        }
    }

    pub fn to_mut_vec(&mut self) -> Result<&mut Vec<Shaun>, ShaunError> {
        match self {
            &mut Shaun::List(ref mut v) => Ok(v),
            _ => Err(ShaunError::NotAList),
        }
    }

    pub fn to_mut_map(&mut self) -> Result<&mut HashMap<String, Shaun>, ShaunError> {
        match self {
            &mut Shaun::Object(ref mut o) => Ok(o),
            _ => Err(ShaunError::NotAnObject),
        }
    }

    pub fn get<T>(&self, id:T) -> Result<& Shaun, ShaunError> where String : From<T> {
        match self {
            &Shaun::Object(ref o) => {
                let s = String::from(id);
                o.get(&s).ok_or_else(move || ShaunError::AttributeNotFound)
            },
            _ => Err(ShaunError::NotAnObject),
        }
    }

    pub fn get_mut<T>(&mut self, id:T) -> Result<&mut Shaun, ShaunError> where String : From<T> {
        match self {
            &mut Shaun::Object(ref mut o) => {
                let s = String::from(id);
                o.get_mut(&s).ok_or_else(move || ShaunError::AttributeNotFound)
            },
            _ => Err(ShaunError::NotAnObject),
        }
    }

    pub fn at<I>(&self, index: I) -> Result<& Shaun, ShaunError> where usize : From<I> {
	let i : usize = index.into();
        match self {
            &Shaun::List(ref v) => {
                if v.len() <= i { Err(ShaunError::OutOfBound) }
                else { Ok(&v[i]) }
            },
            _ => Err(ShaunError::NotAList)
        }
    }

    pub fn at_mut<I>(&mut self, index: I) -> Result<&mut Shaun, ShaunError> where usize : From<I> {
	let i : usize = index.into();
        match self {
            &mut Shaun::List(ref mut v) => {
                if v.len() <= i { Err(ShaunError::OutOfBound) }
                else { Ok(&mut v[i]) }
            },
            _ => Err(ShaunError::NotAList)
        }
    }

    pub fn visit_with<T>(&self, visitor:&mut T) where T : Visitor {
        match self {
            &Shaun::Null => visitor.visit_null(),
            &Shaun::Number(ref v, ref u) => visitor.visit_number(v, u),
            &Shaun::String(ref v) => visitor.visit_string(v),
            &Shaun::Bool(ref v) => visitor.visit_bool(v),
            &Shaun::List(ref l) => visitor.visit_list(l),
            &Shaun::Object(ref o) => visitor.visit_object(o),
        }
    }
}

impl<T : Into<usize>> Index<T> for Shaun {
    type Output = Shaun;
    fn index(&self, index:T) -> &Shaun {
        if let &Shaun::List(ref v) = self {
            let id : usize = index.into();
            &v[id]
        } else {
            &Shaun::Null
        }
    }
}

impl TryInto<f64> for Shaun {
    type Error = ShaunError;
    fn try_into(self) -> Result<f64, ShaunError> {
        match self {
            Shaun::Number(x,_) => Ok(x),
            _ => Err(ShaunError::NotANumber),
        }
    }
}

impl TryInto<String> for Shaun {
    type Error = ShaunError;
    fn try_into(self) -> Result<String, ShaunError> {
        match self {
            Shaun::String(s) => Ok(s),
            _ => Err(ShaunError::NotAString),
        }
    }
}

impl TryInto<bool> for Shaun {
    type Error = ShaunError;
    fn try_into(self) -> Result<bool, ShaunError> {
        match self {
            Shaun::Bool(s) => Ok(s),
            _ => Err(ShaunError::NotABool),
        }
    }
}

impl TryInto<Vec<Shaun>> for Shaun {
    type Error = ShaunError;
    fn try_into(self) -> Result<Vec<Shaun>, ShaunError> {
        match self {
            Shaun::List(v) => Ok(v),
            _ => Err(ShaunError::NotAList),
        }
    }
}

impl TryInto<HashMap<String, Shaun>> for Shaun {
    type Error = ShaunError;
    fn try_into(self) -> Result<HashMap<String, Shaun>, ShaunError> {
        match self {
            Shaun::Object(o) => Ok(o),
            _ => Err(ShaunError::NotAnObject),
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
