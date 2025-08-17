// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};

mod a;
mod b;
mod c;
mod d;

use a::A;
use b::B;
use c::C;
use d::D;

fn main() {

  dtrace_entry_no_nonce("main:::ENTER");
  /* vars */
  dtrace_newline();

  let c = C { cf1: 12, cf2: 14 };
  let b = B { bf1: 23, bf2: 97, bf3: &c };
  let a = A { af1: "hullo", af2: 16, af3: &b };
  foo(&a, 17);

  let arr = [1, 2, 3];
  bar(&arr);

  let c2 = C { cf1: 19, cf2: 10 };
  let c3 = C { cf1: 7, cf2: 61 };
  let c_arr = [&c, &c2, &c3];
  baz(&c_arr);

  let b2 = B { bf1: 6, bf2: 1, bf3: &c2 };
  let b3 = B { bf1: 12, bf2: 11, bf3: &c3 };
  let b_arr = [&b, &b2, &b3];
  foo_bar(&b_arr);

  let d = D { df1: &arr };
  foo_baz(&d);

  let d2 = D { df1: &arr };
  let d3 = D { df1: &arr };
  let d_arr = [&d, &d2, &d3];
  bar_foo(&d_arr);

  dtrace_exit_no_nonce("main:::EXIT1");
  /* vars */
  dtrace_newline();

}

fn foo(a: &A, b: i32) {
  dtrace_entry("foo:::ENTER", *foo_counter.lock().unwrap());
  dtrace_print_pointer(a as *const _ as usize, String::from("a"));
  a.dtrace_print(3, String::from("a"));
  dtrace_print_prim::<i32>(b, String::from("b"));
  dtrace_newline();


  dtrace_exit("foo:::EXIT1", *foo_counter.lock().unwrap());
  dtrace_print_pointer(a as *const _ as usize, String::from("a"));
  a.dtrace_print(3, String::from("a"));
  dtrace_print_prim::<i32>(b, String::from("b"));
  dtrace_newline();
  *foo_counter.lock().unwrap() += 1;
}

fn bar(x: &[i32]) {
  dtrace_entry("bar:::ENTER", *bar_counter.lock().unwrap());
  dtrace_print_pointer(x as *const _ as *const () as usize, String::from("x"));
  dtrace_print_prim_arr::<i32>(&x, String::from("x"));
  dtrace_newline();

  dtrace_exit("bar:::EXIT1", *bar_counter.lock().unwrap());
  dtrace_print_pointer(x as *const _ as *const () as usize, String::from("x"));
  dtrace_print_prim_arr::<i32>(&x, String::from("x"));
  dtrace_newline();
  *bar_counter.lock().unwrap() += 1;
}

fn baz(c_arr: &[&C]) {
  dtrace_entry("baz:::ENTER", *baz_counter.lock().unwrap());
  dtrace_print_pointer(c_arr as *const _ as *const () as usize, String::from("c_arr"));
  C::dtrace_print_arr(&c_arr, String::from("c_arr"));
  dtrace_newline();

  dtrace_exit("baz:::EXIT1", *baz_counter.lock().unwrap());
  dtrace_print_pointer(c_arr as *const _ as *const () as usize, String::from("c_arr"));
  C::dtrace_print_arr(&c_arr, String::from("c_arr"));
  dtrace_newline();
  *baz_counter.lock().unwrap() += 1;
}

fn foo_bar(b_arr: &[&B]) {
  dtrace_entry("foo_bar:::ENTER", *foo_bar_counter.lock().unwrap());
  dtrace_print_pointer(b_arr as *const _ as *const () as usize, String::from("b_arr"));
  B::dtrace_print_arr(&b_arr, String::from("b_arr"));
  dtrace_newline();

  dtrace_exit("foo_bar:::EXIT1", *foo_bar_counter.lock().unwrap());
  dtrace_print_pointer(b_arr as *const _ as *const () as usize, String::from("b_arr"));
  B::dtrace_print_arr(&b_arr, String::from("b_arr"));
  dtrace_newline();
  *foo_bar_counter.lock().unwrap() += 1;
}

fn foo_baz(d: &D) {
  dtrace_entry("foo_baz:::ENTER", *foo_baz_counter.lock().unwrap());
  dtrace_print_pointer(d as *const _ as usize, String::from("d"));
  d.dtrace_print(3, String::from("d"));
  dtrace_newline();

  dtrace_exit("foo_baz:::EXIT1", *foo_baz_counter.lock().unwrap());
  dtrace_print_pointer(d as *const _ as usize, String::from("d"));
  d.dtrace_print(3, String::from("d"));
  dtrace_newline();
  *foo_baz_counter.lock().unwrap() += 1;
}

fn bar_foo(d_arr: &[&D]) {
  dtrace_entry("bar_foo:::ENTER", *bar_foo_counter.lock().unwrap());
  dtrace_print_pointer(d_arr as *const _ as *const () as usize, String::from("d_arr"));
  D::dtrace_print_arr(&d_arr, String::from("d_arr"));
  dtrace_newline();

  dtrace_exit("bar_foo:::EXIT1", *bar_foo_counter.lock().unwrap());
  dtrace_print_pointer(d_arr as *const _ as *const () as usize, String::from("d_arr"));
  D::dtrace_print_arr(&d_arr, String::from("d_arr"));
  dtrace_newline();
  *bar_foo_counter.lock().unwrap() += 1;
}




//== private daikon library ==//

// dtrace File
static tr: LazyLock<Mutex<Option<File>>> = LazyLock::new(||
                                               Mutex::new(dtrace_open()));

// nonce counters
static foo_counter: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static bar_counter: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static baz_counter: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static foo_bar_counter: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static foo_baz_counter: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static bar_foo_counter: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));

fn dtrace_open() -> Option<File> {
  match File::options().append(true).open("main.dtrace") {
    Err(why) => {
      panic!("Daikon couldn't open file, {}", why);
      None
    },
    Ok(traces) => Some(traces),
  }
}

pub fn dtrace_print_pointer_arr<T>(v: &[&T], prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
      let mut arr = String::from("[");
      let mut i = 0;
      while i < v.len()-1 {
        arr.push_str(&format!("0x{:x} ", v[i] as *const _ as usize));
        i += 1;
      }
      if v.len() > 0 {
        arr.push_str(&format!("0x{:x}", v[i] as *const _ as usize));
      }
      arr.push_str("]");
      writeln!(traces, "{}", arr);
      writeln!(traces, "0");
    },
  }
}

// T must implement Display trait
fn dtrace_print_prim_arr<T: std::fmt::Display>(v: &[T], prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
      let mut arr = String::from("[");
      let mut i = 0;
      while i < v.len()-1 {
        arr.push_str(&format!("{} ", v[i]));
        i += 1;
      }
      if v.len() > 0 {
        arr.push_str(&format!("{}", v[v.len()-1]));
      }
      arr.push_str("]");
      writeln!(traces, "{}", arr);
      writeln!(traces, "0");
    },
  };
}

fn dtrace_print_str(v: &str, prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", prefix);
      writeln!(traces, "{}", v);
      writeln!(traces, "0");    
    }
  }
}

// T must implement Display trait
fn dtrace_print_prim<T: std::fmt::Display>(v: T, prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", prefix);
      writeln!(traces, "{}", v);
      writeln!(traces, "0");    
    }
  }
}

fn dtrace_print_pointer(v: usize, prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", prefix);
      writeln!(traces, "0x{:x}", v);
      writeln!(traces, "0");
    },
  };
}

fn dtrace_entry_no_nonce(name: &str) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", name);
    }
  }
}

fn dtrace_exit_no_nonce(name: &str) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", name);
    }
  }
}

fn dtrace_entry(name: &str, nonce: u32) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", name);
      writeln!(traces, "this_invocation_nonce");
      writeln!(traces, "{}", nonce);
    },
  }
}

fn dtrace_exit(name: &str, nonce: u32) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", name);
      writeln!(traces, "this_invocation_nonce");
      writeln!(traces, "{}", nonce);
    },
  }
}

fn dtrace_newline() {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "");
    }
  }
}
