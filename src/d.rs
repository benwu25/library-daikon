// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};
use crate::tr;

pub struct D<'a> {
  pub df1: &'a [i32]
}

impl<'a> D<'a> {
  // rename to dtrace_print_fields
  pub fn dtrace_print(&self, depth: i32, prefix: String) {
    if depth == 0 {
      return;
    }
  
    dtrace_print_prim_arr::<i32>(self.df1, format!("{}{}", prefix, ".df1"));
  }

  pub fn dtrace_print_fields_arr(v: &[&D], depth: i32, prefix: String) {
    if depth == 0 {
      return;
    }

    D::dtrace_print_df1_arr(v, format!("{}{}", prefix, ".df1"));
  }

  // only pointers
  pub fn dtrace_print_df1_arr(v: &[&D], prefix: String) {
    match &mut *tr.lock().unwrap() {
      None => panic!("dtrace file is not open"),
      Some(traces) => {
        writeln!(traces, "{}", prefix.clone());
        let mut arr = String::from("[");
        let mut i = 0;
        while i < v.len()-1 {
          arr.push_str(&format!("0x{:x} ", v[i].df1 as *const _ as *const () as usize));
          i += 1;
        }
        if v.len() > 0 {
          arr.push_str(&format!("0x{:x}", v[v.len()-1].df1 as *const _ as *const () as usize));
        }
        arr.push_str("]");
        writeln!(traces, "{}", arr);
        writeln!(traces, "0");
      }
    }
  }
}


//== daikon private library ==//


// nonce counters
/* none */

pub fn dtrace_print_pointer_arr<T>(v: &[&T], prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "{}", prefix.clone());
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

pub fn dtrace_print_pointer_vec<T>(v: &Vec<&T>, prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "{}", prefix.clone());
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
