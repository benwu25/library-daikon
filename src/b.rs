// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};
use crate::tr;

use crate::c::C;

pub struct B<'a> {
  pub f1: i32,
  pub f2: u16,
  pub f3: &'a C,
}

impl<'a> B<'a> {
  pub fn dtrace_print(&self, depth: i32, prefix: String) {
    if depth == 0 {
      return;
    }
    dtrace_print_prim::<i32>(self.f1, format!("{}{}", prefix, ".f1"));
    dtrace_print_prim::<u16>(self.f2, format!("{}{}", prefix, ".f2"));
    dtrace_print_pointer(self.f3 as *const _ as usize, format!("{}{}", prefix, ".f3"));

    self.f3.dtrace_print(depth - 1, format!("{}{}", prefix, ".f3"));
  }


  //== unfortunate stuff needed for B[]/Vec<B> ==//

  // note how many of these functions are identical except in the way they access v

  pub fn dtrace_print_arr(v: &[&B], prefix: String) {
    dtrace_print_pointer_arr::<B>(&v, prefix.clone());

    // Proposed fix: make a Vec<&C> and call C::dtrace_print_arr().
    let mut ref_arr = Vec::new();
    let mut i = 0;
    while i < v.len() {
      ref_arr.push(v[i].f3);
      i += 1;
    }

    B::dtrace_print_f1_arr(&v, format!("{}{}", prefix, ".f1"));
    B::dtrace_print_f2_arr(&v, format!("{}{}", prefix, ".f2"));
    C::dtrace_print_fields_vec(&ref_arr, format!("{}{}", prefix, ".f3"));
  }

  pub fn dtrace_print_f1_arr(v: &[&B], prefix: String) {
    match &mut *tr.lock().unwrap() {
      None => panic!("dtrace file is not open"),
      Some(traces) => {
        writeln!(traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
        let mut arr = String::from("[");
        let mut i = 0;
        while i < v.len()-1 {
          arr.push_str(&format!("{} ", v[i].f1));
          i += 1;
        }
        if v.len() > 0 {
          arr.push_str(&format!("{}", v[v.len()-1].f1));
        }
        arr.push_str("]");
        writeln!(traces, "{}", arr);
        writeln!(traces, "0");
      },
    }
  }

  pub fn dtrace_print_f2_arr(v: &[&B], prefix: String) {
    match &mut *tr.lock().unwrap() {
      None => panic!("dtrace file is not open"),
      Some(traces) => {
        writeln!(traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
        let mut arr = String::from("[");
        let mut i = 0;
        while i < v.len()-1 {
          arr.push_str(&format!("{} ", v[i].f2));
          i += 1;
        }
        if v.len() > 0 {
          arr.push_str(&format!("{}", v[v.len()-1].f2));
        }
        arr.push_str("]");
        writeln!(traces, "{}", arr);
        writeln!(traces, "0");
      },
    }
  }
} // impl B


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
