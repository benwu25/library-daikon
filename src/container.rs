// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};
use crate::tr;

use crate::dummies::{X,Y,Z};

pub struct Container<'a> {
  pub contf1: &'a X,
  pub contf2: &'a Y,
  pub contf3: &'a Z,
}

impl<'a> Container<'a> {
  pub fn dtrace_print_fields_vec(v: &Vec<&Container>, depth: i32, prefix: String) {
    if depth == 0 {
      return;
    }

    let mut x_ref_arr = Vec::new();
    let mut y_ref_arr = Vec::new();
    let mut z_ref_arr = Vec::new();
    let mut i = 0;
    while i < v.len() {
      x_ref_arr.push(v[i].contf1);
      y_ref_arr.push(v[i].contf2);
      z_ref_arr.push(v[i].contf3);
      i += 1;
    }

    dtrace_print_pointer_vec::<X>(&x_ref_arr, format!("{}{}", prefix, ".contf1"));
    X::dtrace_print_fields_vec(&x_ref_arr, depth - 1, format!("{}{}", prefix, ".contf1"));
    dtrace_print_pointer_vec::<Y>(&y_ref_arr, format!("{}{}", prefix, ".contf2"));
    Y::dtrace_print_fields_vec(&y_ref_arr, depth - 1, format!("{}{}", prefix, ".contf2"));
    dtrace_print_pointer_vec::<Z>(&z_ref_arr, format!("{}{}", prefix, ".contf3"));
    Z::dtrace_print_fields_vec(&z_ref_arr, depth - 1, format!("{}{}", prefix, ".contf3"));
  }
}

pub fn dtrace_print_pointer_vec<T>(v: &Vec<&T>, var_name: String) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "{}", var_name);
      let mut arr = String::from("[");
      let mut i = 0;
      while i < v.len()-1 {
        arr.push_str(&format!("0x{:x} ", v[i] as *const _ as usize));
        i += 1;
      }
      if v.len() > 0 {
        arr.push_str(&format!("0x{:x}", v[v.len()-1] as *const _ as usize));
      }
      arr.push_str("]");
      writeln!(traces, "{}", arr);
      writeln!(traces, "0");
    },
  }
}

