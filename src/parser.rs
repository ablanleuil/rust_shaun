use shaun_type::Shaun;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashMap;
use std::iter::Peekable;
use std::vec::Vec;

static mut COL : u32 = 0;
static mut ROW : u32 = 0;

fn advance<I>(stream : &mut Peekable<I>) where I : Iterator<Item=char> {
    unsafe {
        if let Some('\n') = stream.next() {
            COL = 0;
            ROW = ROW + 1;
        } else {
            COL = COL + 1;
        }
    }
}

#[derive(Debug, PartialEq)]
enum ParseToken {
    Key(String),
    Id(String),
    Atom(ParseAtom),
    Comment(String),
}

#[derive(Debug, PartialEq)]
enum ParseAtom {
    String(String),
    Bool(bool),
    Double(f64),
    Null,
}

/*
 * SlideWindow represents a fixed size sliding [char]:
 *
 * let window : SlideWindow = SlideWindow::new(3)
 * window.push('a')    //  "a"
 * window.push('b')    //  "ab"
 * window.push('c')    //  "bc"
 * window.push('d')    //  "cd"
 */
#[derive(Debug)]
struct SlideWindow {
    arr : Vec<char>,
    id  : usize,
}

impl SlideWindow {
    pub fn new(n:usize) -> SlideWindow { let mut v = Vec::new(); v.resize(n, '\0'); SlideWindow { arr: v, id : 0 } }

    pub fn push(&mut self, e:char) -> () {
        println!("{:?}", self);
        let len = self.arr.len();
        if self.id < len { self.arr[self.id] = e; self.id = self.id+1; return; }
        for i in 1..len {
            self.arr[i-1] = self.arr[i].clone();
        }
        self.arr[len-1] = e
    }

    pub fn first(&self) -> char {
        self.arr[0]
    }
}

impl PartialEq<String> for SlideWindow {
    fn eq(&self, other:&String) -> bool {
        let vec : Vec<char> = other.chars().collect();
        self.arr == vec
    }

    fn ne(&self, other:&String) -> bool {
        let vec : Vec<char> = other.chars().collect();
        self.arr != vec
    }
}

/*
 * Helper functions, mostly for testing characters
 */

fn is_kwd(c:char) -> bool {
    c == ':' ||
    c == '{' ||
    c == '}' ||
    c == '[' ||
    c == ']'
}

fn is_id_begin(c:char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_comment_begin(c:char) -> bool {
    c == '/' || c == '#' || c == '('
}

fn is_double_begin(c:char) -> bool {
    c.is_digit(10) || c == '.' || c == '-' || c == '+'
}

fn is_id_char(c:char) -> bool {
    c.is_digit(10) || is_id_begin(c)
}

fn is_double_char(c:char) -> bool {
    c.is_digit(10) || c == '.' || c == '-'
        || c == '+' || c == 'e' || c == 'E'
}

/**
 * lex_<x> functions are functions which consume a stream
 * of char and return a value depending on the object we are
 * "lexing"
 *
 * every lex_<x> function must consume the stream to the caracter just after
 * the read symbol.
 *
 * "10 years" -> lex_number() -> "10 years"
 *  ^                               ^
 *  before                        after
 */
fn lex_number<I>(stream : &mut Peekable<I>) -> f64
where I : Iterator<Item=char> {
    let mut buffer = String::new();

    while stream.peek().is_some() {
        let c = *stream.peek().unwrap();
        if is_double_char(c) { buffer.push(c) }
        else { break }

        advance(stream);
    }

    buffer.parse().unwrap()
}

fn lex_id<I>(stream : &mut Peekable<I>) -> String
where I : Iterator<Item=char> {
    let mut buffer = String::new();

    while stream.peek().is_some() {
        let c = *stream.peek().unwrap();
        if is_id_char(c) { buffer.push(c) }
        else { break }

        advance(stream);
    }

    buffer
}


fn lex_comment<I>(stream : &mut Peekable<I>) -> String
where I : Iterator<Item=char> {
    let mut buffer = String::new();
    let mut w : SlideWindow;
    let end : String;

    match stream.peek() {
        None => return buffer,
        Some(&'/') => {
            advance(stream);
            match stream.peek() {
                None => return buffer,
                Some(&'*') => { w = SlideWindow::new(2); end = String::from("*/") },
                Some(&'/') => { w = SlideWindow::new(1); end = String::from("\n") },
                _ => return buffer
            }
        }
        Some(&'(') => { w = SlideWindow::new(1); end = String::from(")") },
        Some(&'#') => { w = SlideWindow::new(1); end = String::from("\n") },
        _ => return buffer
    }

    advance(stream);
    while stream.peek().is_some() {
        let c = *stream.peek().unwrap();
        w.push(c);
        if w == end { break }
        buffer.push(w.first());
        advance(stream);
    }

    advance(stream);
    buffer
}

fn lex_string<I>(stream : &mut Peekable<I>) -> String
where I : Iterator<Item=char> {
    let mut buffer = String::new();
    advance(stream);

    if let Some(&'\n') = stream.peek() {
        advance(stream);
    }

    while stream.peek().is_some() {
        let c = *stream.peek().unwrap();
        if c == '"' { break }

        if c == '\\' {
            advance(stream);
            let c2 = *stream.peek().unwrap();
            buffer.push(match c2 {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                _   => c2,
            })
        }
        else {
            buffer.push(c)
        }

        advance(stream);
    }
    
    advance(stream);
    if let Some('\n') = buffer.chars().rev().next() { buffer.pop(); }
    buffer
}

/*
 * lex(str) transforms a str into a token vector
 */
fn lex<I>(it:&mut Peekable<I>) -> Vec<ParseToken>
where I : Iterator<Item=char> {
    unsafe {
        COL = 0;
        ROW = 0;
    }
    let mut ret = Vec::new();

    loop {
        match it.peek() { None => break, Some(&c) => {
                // ID case
                // also manages "ID-like keywords" such as true, false, or null
                if is_id_begin(c) {
                    let id = lex_id(it);
                    if id == "true" { ret.push(ParseToken::Atom(ParseAtom::Bool(true))) }
                    else if id == "false" { ret.push(ParseToken::Atom(ParseAtom::Bool(false))) }
                    else if id == "null" { ret.push(ParseToken::Atom(ParseAtom::Null)) }
                    else { ret.push(ParseToken::Id(id)) }
                }

                // Double case
                // manages f64 numbers
                else if is_double_begin(c) {
                    let dbl = lex_number(it);
                    ret.push(ParseToken::Atom(ParseAtom::Double(dbl)))
                }

                // Keyword case
                // manages every 1 character keys (mostly delimiters such as '{' '}')
                else if is_kwd(c) {
                    ret.push(ParseToken::Key(c.to_string()));
                    it.next();
                }

                // Comment case
                // comments are embeded in the Shaun datastructure
                // for dumping them into a file, if wanted
                else if is_comment_begin(c) {
                    let comment = lex_comment(it);
                    ret.push(ParseToken::Comment(comment))
                }

                // String case
                else if c == '"' {
                    let string = lex_string(it);
                    ret.push(ParseToken::Atom(ParseAtom::String(string)))
                }
                else {
                    it.next();
                }
            }
        }
    }

    ret
}

fn atom_to_value(toks : &Vec<ParseToken>, i : &mut usize) -> Shaun {
    let ret = match toks[*i] {
        ParseToken::Atom(ParseAtom::Double(ref f)) => {
            let id : String;
            if toks.len() > *i+1 {
                id = match toks[*i+1] {
                    ParseToken::Id(ref unit) => { *i = *i + 1; unit.clone() },
                    _ => String::new()
                };
            } else { id = String::new() }
            Shaun::Number(*f, id)
        },
        ParseToken::Atom(ParseAtom::String(ref s)) => {
            Shaun::String(s.clone())
        },
        ParseToken::Atom(ParseAtom::Bool(ref b)) => {
            Shaun::Bool(*b)
        },
        _ => {
            Shaun::Null
        }
    };

    *i = *i + 1;
    ret
}

fn parse_name(toks : &Vec<ParseToken>, i : &mut usize) -> String {
    match toks[*i] {
        ParseToken::Id(ref name) => { *i = *i + 1; name.clone() },
        _ => String::new(),
    }
}

fn parse_object(toks : &Vec<ParseToken>, i : &mut usize) -> Shaun {
    *i = *i + 1;
    let ret = parse_raw_object(toks, i);
    *i = *i + 1;

    ret
}


fn parse_raw_object(toks : &Vec<ParseToken>, i : &mut usize) -> Shaun {
    let mut obj = HashMap::new();
    while let ParseToken::Id(_) = toks[*i] {
        let name = parse_name(toks, i);
        *i = *i + 1;
        let value = parse_value(toks, i);
        println!("parsed object attrib {:?} {:?}", name, value);
        obj.insert(name, value);
        if *i >= toks.len() { break }
    }

    Shaun::Object(obj)
}

fn parse_list(toks : &Vec<ParseToken>, i : &mut usize) -> Shaun {
    let mut list = Vec::new();

    *i = *i + 1;
    while toks[*i] != ParseToken::Key(']'.to_string()) {
        list.push(parse_value(toks, i))
    }

    *i = *i + 1;

    Shaun::List(list)
}

fn parse_value(toks : &Vec<ParseToken>, i : &mut usize) -> Shaun {
    match toks[*i] {
        ParseToken::Atom(ref _a) => atom_to_value(toks, i),
        ParseToken::Key(ref k) => {
            if *k == '{'.to_string() {
                parse_object(toks, i)
            } else if *k == '['.to_string() {
                parse_list(toks, i)
            } else { Shaun::Null }
        }
        _ => Shaun::Null
    }
}

fn parse_all(toks : &Vec<ParseToken>, i : &mut usize) -> Shaun {
    match toks[*i] {
        ParseToken::Key(ref k) => { if *k == '{'.to_string() { parse_object(toks, i) } else { Shaun::Null } },
        _ => { parse_raw_object(toks, i) },
    }
}

/*
 * parse_str(str) parses a str to a Shaun value.
 * it works as follow:
 *
 * let vec = lex(str);
 * let val = parse(vec);
 * return val
 *
 */
pub fn parse_str<'a>(s:&'a str) -> Shaun {
    let mut it = s.chars().peekable();
    let vec = lex(&mut it);
    parse_all(&vec, &mut 0)
}

// String version of parse_str()
pub fn parse_string(s:String) -> Shaun {
    let mut it = s.chars().peekable();
    let vec = lex(&mut it);
    parse_all(&vec, &mut 0)
}

pub fn parse_file(filepath:&Path) -> Shaun {
    let mut s  = String::new();
    let mut f = File::open(filepath)
        .expect("Shaun::parse_file(): Something went wrong while opening file.");

    f.read_to_string(&mut s)
        .expect("Shaun::parse_file(): Something went wrong while reading file.");
    let mut it = s.chars().peekable();

    let vec = lex(&mut it);
    for e in &vec {
        println!("{:?}", e)
    }
    parse_all(&vec, &mut 0)
}
