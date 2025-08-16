// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};

mod a;
mod b;
mod c;

use a::A;
use b::B;
use c::C;

fn main() {

  dtrace_entry_no_nonce("main:::ENTER");
  /* vars */
  dtrace_newline();

  let c = C { f1: 12, f2: 14 };
  let b = B { f1: 23, f2: 97, f3: &c };
  let a = A { f1: "hullo", f2: 16, f3: &b };
  foo(&a, 17);

  let arr = [1, 2, 3];
  bar(&arr);

  let c2 = C { f1: 19, f2: 10 };
  let c3 = C { f1: 7, f2: 61 };
  let c_arr = [&c, &c2, &c3];
  baz(&c_arr);

  let b2 = B { f1: 6, f2: 1, f3: &c2 };
  let b3 = B { f1: 12, f2: 11, f3: &c3 };
  let b_arr = [&b, &b2, &b3];
  foo_bar(&b_arr);

  dtrace_exit_no_nonce("main:::EXIT1");
  /* vars */
  dtrace_newline();

}

fn foo(a: &A, b: i32) {
  dtrace_entry("foo:::ENTER", *foo_counter.lock().unwrap());
  a.dtrace_print(3, String::from("a"));
  dtrace_print_prim::<i32>(b, String::from("b"));
  dtrace_newline();


  dtrace_exit("foo:::EXIT1", *foo_counter.lock().unwrap());
  a.dtrace_print(3, String::from("a"));
  dtrace_print_prim::<i32>(b, String::from("b"));
  dtrace_newline();
  *foo_counter.lock().unwrap() += 1;
}

fn bar(x: &[i32]) {
  dtrace_entry("bar:::ENTER", *bar_counter.lock().unwrap());
  dtrace_print_prim_arr::<i32>(&x, String::from("x"));
  dtrace_newline();

  dtrace_exit("bar:::EXIT1", *bar_counter.lock().unwrap());
  dtrace_print_prim_arr::<i32>(&x, String::from("x"));
  dtrace_newline();
  *bar_counter.lock().unwrap() += 1;
}

fn baz(c_arr: &[&C]) {
  dtrace_entry("baz:::ENTER", *baz_counter.lock().unwrap());
  C::dtrace_print_arr(&c_arr, String::from("c_arr"));
  dtrace_newline();

  dtrace_exit("baz:::EXIT1", *baz_counter.lock().unwrap());
  C::dtrace_print_arr(&c_arr, String::from("c_arr"));
  dtrace_newline();
  *baz_counter.lock().unwrap() += 1;
}

fn foo_bar(b_arr: &[&B]) {
  dtrace_entry("foo_bar:::ENTER", *foo_bar_counter.lock().unwrap());
  B::dtrace_print_arr(&b_arr, String::from("b_arr"));
  dtrace_newline();

  dtrace_exit("foo_bar:::EXIT1", *foo_bar_counter.lock().unwrap());
  B::dtrace_print_arr(&b_arr, String::from("b_arr"));
  dtrace_newline();
  *foo_bar_counter.lock().unwrap() += 1;
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

fn dtrace_open() -> Option<File> {
  match File::options().append(true).open("main.dtrace") {
    Err(why) => {
      panic!("Daikon couldn't open file, {}", why);
      None
    },
    Ok(traces) => Some(traces),
  }
}

// T must implement Display trait
fn dtrace_print_prim_arr<T: std::fmt::Display>(v: &[T], prefix: String) {
  // should be relocated as Mike suggested
  dtrace_print_pointer(v as *const _ as *const () as usize, prefix.clone());

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
