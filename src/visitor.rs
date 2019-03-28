use shaun_type::Shaun;

use std::collections::HashMap;

pub trait Visitor {
    fn visit_null(&mut self);
    fn visit_number(&mut self, value:&f64, unit:&String);
    fn visit_string(&mut self, value:&String);
    fn visit_bool(&mut self, value:&bool);
    fn visit_list(&mut self, list:&Vec<Shaun>);
    fn visit_object(&mut self, object:&HashMap<String,Shaun>);
}

pub struct PrettyPrinter {
  level : usize,
}

impl PrettyPrinter {
  pub fn new() -> PrettyPrinter {
    PrettyPrinter { level:0 }
  }

  fn spaces(&self) {
    print!("{}", " ".repeat(self.level))
  }
}

impl Visitor for PrettyPrinter {
  fn visit_null(&mut self) { print!("null") }
  fn visit_number(&mut self, value:&f64, unit:&String) {
    print!("{} {}", value, unit)
  }

  fn visit_string(&mut self, value:&String) {
    print!("\"{}\"", value)
  }

  fn visit_bool(&mut self, value:&bool) {
    print!("{}", value)
  }

  fn visit_list(&mut self, list:&Vec<Shaun>) {
    println!("[ ");
    self.level += 2;
    for sn in list.iter() {
      self.spaces();
      sn.visit_with(self);
    }
    self.level -= 2;
    self.spaces(); print!("]")
  }

  fn visit_object(&mut self, map:&HashMap<String, Shaun>) {
    println!("{{ ");
    self.level += 2;
    for (key, sn) in map.iter() {
      self.spaces();
      print!("{}: ", key);
      let kl = key.len();
      self.level += kl + 2;
      sn.visit_with(self);
      self.level -= kl + 2;
    }
    self.level -= 2;
    self.spaces(); print!("}}")
  }
}
