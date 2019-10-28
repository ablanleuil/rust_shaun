use shaun_type::Shaun;

use std::collections::HashMap;
use std::io::Write;

pub trait Visitor {
    fn visit_null(&mut self);
    fn visit_number(&mut self, value:&f64, unit:&String);
    fn visit_string(&mut self, value:&String);
    fn visit_bool(&mut self, value:&bool);
    fn visit_list(&mut self, list:&Vec<Shaun>);
    fn visit_object(&mut self, object:&HashMap<String,Shaun>);
}

pub struct PrettyPrinter<'a, T : Write> {
  level : usize,
  buffer : &'a mut T,
}

impl<'a, T : Write> PrettyPrinter<'a, T> {
  pub fn to(b :&'a mut T) -> PrettyPrinter<'a, T> {
    PrettyPrinter { level:0, buffer:b }
  }

  fn spaces(&mut self) {
    write!(self.buffer, "{}", " ".repeat(self.level)).unwrap();
  }

  pub fn result(&self) -> &T {
      self.buffer
  }
}

impl<'a, T : Write> Visitor for PrettyPrinter<'a, T> {
  fn visit_null(&mut self) { write!(self.buffer, "null").unwrap() }
  fn visit_number(&mut self, value:&f64, unit:&String) {
    write!(self.buffer, "{} {}", value, unit).unwrap()
  }

  fn visit_string(&mut self, value:&String) {
    let value = value.replace("\\", "\\\\");
    let value = value.replace("\"", "\\\"");
    write!(self.buffer, "\"{}\"", value).unwrap()
  }

  fn visit_bool(&mut self, value:&bool) {
    write!(self.buffer, "{}", value).unwrap()
  }

  fn visit_list(&mut self, list:&Vec<Shaun>) {
    writeln!(self.buffer, "[ ").unwrap();
    self.level += 2;
    for sn in list.iter() {
      self.spaces();
      sn.visit_with(self);
    }
    self.level -= 2;
    self.spaces(); write!(self.buffer, "]").unwrap()
  }

  fn visit_object(&mut self, map:&HashMap<String, Shaun>) {
    writeln!(self.buffer, "{{ ").unwrap();
    self.level += 2;
    for (key, sn) in map.iter() {
      self.spaces();
      write!(self.buffer, "{}: ", key).unwrap();
      let kl = key.len();
      self.level += kl + 2;
      sn.visit_with(self);
      self.level -= kl + 2;
    }
    self.level -= 2;
    self.spaces(); write!(self.buffer, "}}").unwrap()
  }
}
