// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};
use crate::tr;

use crate::B;

pub struct A<'a> {
  pub af1: &'a str,
  pub af2: u32,
  pub af3: &'a B<'a>,
}

impl<'a> A<'a> {
  pub fn dtrace_print(&self, depth: i32, prefix: String) {
    if depth == 0 {
      return;
    }
    dtrace_print_str(self.af1, format!("{}{}", prefix, ".af1"));
    dtrace_print_prim::<u32>(self.af2, format!("{}{}", prefix, ".af2"));
    dtrace_print_pointer(self.af3 as *const _ as usize, format!("{}{}", prefix, ".af3"));
    self.af3.dtrace_print(depth - 1, format!("{}{}", prefix, ".af3"));
  }
}


//== daikon private library ==//


// nonce counters
/* none */

pub fn dtrace_print_pointer_arr<T>(v: &[&T], prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
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
  dtrace_print_pointer(v as *const _ as *const () as usize, prefix.clone());
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
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
  }
}

fn dtrace_print_str(v: &str, prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "{}", prefix);
      writeln!(traces, "\"{}\"", v);
      writeln!(traces, "0");
    },
  }
}

// T must implement Display trait
fn dtrace_print_prim<T: std::fmt::Display>(v: T, prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "{}", prefix);
      writeln!(traces, "{}", v);
      writeln!(traces, "0");
    },
  }
}

fn dtrace_print_pointer(v: usize, prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "{}", prefix);
      writeln!(traces, "0x{:x}", v);
      writeln!(traces, "0");
    },
  }
}

fn dtrace_entry_no_nonce(name: &str) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "{}", name);
    },
  }
}

fn dtrace_exit_no_nonce(name: &str) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "{}", name);
    },
  }
}

fn dtrace_entry(name: &str, nonce: u32) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "{}", name);
      writeln!(traces, "this_invocation_nonce");
      writeln!(traces, "{}", nonce);
    },
  }
}

fn dtrace_exit(name: &str, nonce: u32) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "{}", name);
      writeln!(traces, "this_invocation_nonce");
      writeln!(traces, "{}", nonce);
    },
  }
}

fn dtrace_newline() {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "");
    },
  }
}
