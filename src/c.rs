// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};
use crate::tr;

pub struct C {
  pub cf1: i16,
  pub cf2: i32,
  // pub f3: array or vec
}

impl C {
  pub fn dtrace_print(&self, depth: i32, prefix: String) {
    if depth == 0 {
      return;
    }
    dtrace_print_prim::<i16>(self.cf1, format!("{}{}", prefix, ".cf1"));
    dtrace_print_prim::<i32>(self.cf2, format!("{}{}", prefix, ".cf2"));
  }

  // don't care about addr of v or addr of C's
  pub fn dtrace_print_fields_vec(v: &Vec<&C>, prefix: String) {
    C::dtrace_print_f1_vec(v, format!("{}{}", prefix, ".cf1"));
    C::dtrace_print_f2_vec(v, format!("{}{}", prefix, ".cf2"));
  }

  pub fn dtrace_print_arr(v: &[&C], prefix: String) {
    dtrace_print_pointer_arr::<C>(&v, prefix.clone());
    C::dtrace_print_f1_arr(&v, format!("{}{}", prefix, "[..].cf1"));
    C::dtrace_print_f2_arr(&v, format!("{}{}", prefix, "[..].cf2"));
  }

  // TODO
  pub fn dtrace_print_pointer_vec(v: &Vec<&C>, prefix: String) {
    match &mut *tr.lock().unwrap() {
      None => panic!("dtrace file is not open"),
      Some(traces) => {
        writeln!(traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
        let mut arr = String::from("[");
        let mut i = 0;
        while i < v.len() - 1 {
          arr.push_str(&format!("0x{:x} ", v[i] as *const _ as usize));
          i += 1;
        }
        if v.len() > 0 {
          arr.push_str(&format!("0x{:x}", v[v.len() - 1] as *const _ as usize));
        }
        arr.push_str("]");
        writeln!(traces, "{}", arr);
        writeln!(traces, "0");
      },
    }
  }

  pub fn dtrace_print_f1_vec(v: &Vec<&C>, prefix: String) {
    match &mut *tr.lock().unwrap() {
      None => panic!("dtrace file is not open"),
      Some(traces) => {
        writeln!(traces, "{}", prefix.clone());
        let mut arr = String::from("[");
        let mut i = 0;
        while i < v.len()-1 {
          arr.push_str(&format!("{} ", v[i].cf1));
          i += 1;
        }
        if v.len() > 0 {
          arr.push_str(&format!("{}", v[v.len() - 1].cf1));
        }
        arr.push_str("]");
        writeln!(traces, "{}", arr);
        writeln!(traces, "0");
      },
    }
  }

  // like dtrace_print_prim_arr but v[i].f1 instead of v[i]
  pub fn dtrace_print_f1_arr(v: &[&C], prefix: String) {
    match &mut *tr.lock().unwrap() {
      None => panic!("dtrace file is not open"),
      Some(traces) => {
        writeln!(traces, "{}", prefix.clone());
        let mut arr = String::from("[");
        let mut i = 0;
        while i < v.len()-1 {
          arr.push_str(&format!("{} ", v[i].cf1));
          i += 1;
        }
        if v.len() > 0 {
          arr.push_str(&format!("{}", v[v.len()-1].cf1));
        }
        arr.push_str("]");
        writeln!(traces, "{}", arr);
        writeln!(traces, "0");
      },
    }
  }

  // TODO
  pub fn dtrace_print_f2_vec(v: &Vec<&C>, prefix: String) {
    match &mut *tr.lock().unwrap() {
      None => panic!("dtrace file is not open"),
      Some(traces) => {
        writeln!(traces, "{}", prefix.clone());
        let mut arr = String::from("[");
        let mut i = 0;
        while i < v.len()-1 {
          arr.push_str(&format!("{} ", v[i].cf2));
          i += 1;
        }
        if v.len() > 0 {
          arr.push_str(&format!("{}", v[v.len()-1].cf2));
        }
        arr.push_str("]");
        writeln!(traces, "{}", arr);
        writeln!(traces, "0");
      },
    }
  }

  // v[i].f2 instead of v[i]
  pub fn dtrace_print_f2_arr(v: &[&C], prefix: String) {
    match &mut *tr.lock().unwrap() {
      None => panic!("dtrace file is not open"),
      Some(traces) => {
        writeln!(traces, "{}", prefix.clone());
        let mut arr = String::from("[");
        let mut i = 0;
        while i < v.len()-1 {
          arr.push_str(&format!("{} ", v[i].cf2));
          i += 1;
        }
        if v.len() > 0 {
          arr.push_str(&format!("{}", v[v.len()-1].cf2));
        }
        arr.push_str("]");
        writeln!(traces, "{}", arr);
        writeln!(traces, "0");
      },
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
