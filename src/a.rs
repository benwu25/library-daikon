// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};

use crate::B;

pub struct A<'a> {
  pub f1: &'a str,
  pub f2: u32,
  pub f3: &'a B<'a>,
}

impl<'a> A<'a> {
  pub fn dtrace_print(&self, depth: i32, prefix: String) {
    if depth == 0 {
      return;
    }
    dtrace_print_pointer(self as *const _ as usize, prefix.clone());
    dtrace_print_str(self.f1, format!("{}{}", prefix, ".f1"));
    dtrace_print_prim::<u32>(self.f2, format!("{}{}", prefix, ".f2"));
    self.f3.dtrace_print(depth - 1, format!("{}{}", prefix, ".f3"));
  }
}


//== daikon private library ==//


// nonce counters
/* none */

// T must implement Display trait
fn dtrace_print_prim_arr<T: std::fmt::Display>(v: &[T], prefix: String) {
  let mut traces = match File::options().append(true).open("main.dtrace") {
    Err(why) => panic!("Daikon couldn't open file, {}", why),
    Ok(traces) => traces,
  };
  dtrace_print_pointer(v as *const _ as *const () as usize, prefix.clone());
  writeln!(&mut traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
  let mut arr = String::from("[");
  let mut i = 0;
  while i < v.len()-1 {
    arr.push_str(&format!("{} ", v[i]));
    i += 1;
  }
  if arr.len() > 0 {
    arr.push_str(&format!("{}", v[v.len()-1]));
  }
  arr.push_str("]");
  writeln!(&mut traces, "{}", arr);
  writeln!(&mut traces, "0");
}

fn dtrace_print_str(v: &str, prefix: String) {
  let mut traces = match File::options().append(true).open("main.dtrace") {
    Err(why) => panic!("Daikon couldn't open file, {}", why),
    Ok(traces) => traces,
  };
  writeln!(&mut traces, "{}", prefix);
  writeln!(&mut traces, "\"{}\"", v);
  writeln!(&mut traces, "0");
}

// T must implement Display trait
fn dtrace_print_prim<T: std::fmt::Display>(v: T, prefix: String) {
  let mut traces = match File::options().append(true).open("main.dtrace") {
    Err(why) => panic!("Daikon couldn't open file, {}", why),
    Ok(traces) => traces,
  };
  writeln!(&mut traces, "{}", prefix);
  writeln!(&mut traces, "{}", v);
  writeln!(&mut traces, "0");
}

fn dtrace_print_pointer(v: usize, prefix: String) {
  let mut traces = match File::options().append(true).open("main.dtrace") {
    Err(why) => panic!("Daikon couldn't open file, {}", why),
    Ok(traces) => traces,
  };
  writeln!(&mut traces, "{}", prefix);
  writeln!(&mut traces, "0x{:x}", v);
  writeln!(&mut traces, "0");
}

fn dtrace_entry(name: &str, nonce: u32) {
  let mut traces = match File::options().append(true).open("main.dtrace") {
    Err(why) => panic!("Daikon couldn't open file, {}", why),
    Ok(traces) => traces,
  };
  writeln!(&mut traces, "{}", name);
  writeln!(&mut traces, "this_invocation_nonce");
  writeln!(&mut traces, "{}", nonce);
}

fn dtrace_exit(name: &str, nonce: u32) {
  let mut traces = match File::options().append(true).open("main.dtrace") {
    Err(why) => panic!("Daikon couldn't open file, {}", why),
    Ok(traces) => traces,
  };
  writeln!(&mut traces, "{}", name);
  writeln!(&mut traces, "this_invocation_nonce");
  writeln!(&mut traces, "{}", nonce);
}

fn dtrace_newline() {
  let mut traces = match File::options().append(true).open("main.dtrace") {
    Err(why) => panic!("Daikon couldn't open file, {}", why),
    Ok(traces) => traces,
  };
  writeln!(&mut traces, "");
}
