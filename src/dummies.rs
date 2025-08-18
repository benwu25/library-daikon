// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};
use crate::tr;

pub struct X {
  pub xf1: i32,
}

pub struct Y {
  pub yf1: i32,
}

pub struct Z {
  pub zf1: i32,
}

impl X {
  pub fn dtrace_print_fields_vec(v: &Vec<&X>, depth: i32, prefix: String) {
    if depth == 0 {
      return;
    }

    X::dtrace_print_xf1_vec(v, format!("{}{}", prefix, ".xf1"));
  }

  pub fn dtrace_print_xf1_vec(v: &Vec<&X>, var_name: String) {
    match &mut *tr.lock().unwrap() {
      None => panic!("dtrace file is not open"),
      Some(traces) => {
        writeln!(traces, "{}", var_name).ok();
        let mut arr = String::from("[");
        let mut i = 0;
        while i < v.len()-1 {
          arr.push_str(&format!("{} ", v[i].xf1));
          i += 1;
        }
        if v.len() > 0 {
          arr.push_str(&format!("{}", v[v.len()-1].xf1));
        }
        arr.push_str("]");
        writeln!(traces, "{}", arr).ok();
        writeln!(traces, "0").ok();
      },
    }
  }
}

impl Y {
  pub fn dtrace_print_fields_vec(v: &Vec<&Y>, depth: i32, prefix: String) {
    if depth == 0 {
      return;
    }

    Y::dtrace_print_yf1_vec(v, format!("{}{}", prefix, ".yf1"));
  }

  pub fn dtrace_print_yf1_vec(v: &Vec<&Y>, var_name: String) {
    match &mut *tr.lock().unwrap() {
      None => panic!("dtrace file is not open"),
      Some(traces) => {
        writeln!(traces, "{}", var_name).ok();
        let mut arr = String::from("[");
        let mut i = 0;
        while i < v.len()-1 {
          arr.push_str(&format!("{} ", v[i].yf1));
          i += 1;
        }
        if v.len() > 0 {
          arr.push_str(&format!("{}", v[v.len()-1].yf1));
        }
        arr.push_str("]");
        writeln!(traces, "{}", arr).ok();
        writeln!(traces, "0").ok();
      },
    }
  }

}

impl Z {
  pub fn dtrace_print_fields_vec(v: &Vec<&Z>, depth: i32, prefix: String) {
    if depth == 0 {
      return;
    }

    Z::dtrace_print_zf1_vec(v, format!("{}{}", prefix, ".zf1"));
  }

  pub fn dtrace_print_zf1_vec(v: &Vec<&Z>, var_name: String) {
    match &mut *tr.lock().unwrap() {
      None => panic!("dtrace file is not open"),
      Some(traces) => {
        writeln!(traces, "{}", var_name).ok();
        let mut arr = String::from("[");
        let mut i = 0;
        while i < v.len()-1 {
          arr.push_str(&format!("{} ", v[i].zf1));
          i += 1;
        }
        if v.len() > 0 {
          arr.push_str(&format!("{}", v[v.len()-1].zf1));
        }
        arr.push_str("]");
        writeln!(traces, "{}", arr).ok();
        writeln!(traces, "0").ok();
      },
    }
  }
}
