// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};

use crate::c::C;

pub struct B<'a> {
  pub f1: i32,
  pub f2: u16,
  pub f3: &'a C,
}

impl<'a> B<'a> {
  pub fn dtrace_print(&self, depth: i32, prefix: String) {
    dtrace_print_pointer(self as *const _ as usize, prefix.clone());
    if depth == 0 {
      return;
    }
    dtrace_print_prim::<i32>(self.f1, format!("{}{}", prefix, ".f1"));
    dtrace_print_prim::<u16>(self.f2, format!("{}{}", prefix, ".f2"));
    self.f3.dtrace_print(depth - 1, format!("{}{}", prefix, ".f3"));
  }


  //== unfortunate stuff needed for B[]/Vec<B> ==//

  // note how many of these functions are identical except in the way they access v

  pub fn dtrace_print_arr(v: &[&B], prefix: String) {
    dtrace_print_pointer(v as *const _ as *const () as usize, prefix.clone());

    B::dtrace_print_pointer_arr(&v, prefix.clone());

    // Proposed fix: make a Vec<&C> and call C::dtrace_print_arr().
    let mut ref_arr = Vec::new();
    let mut i = 0;
    while i < v.len() {
      ref_arr.push(v[i].f3);
      i += 1;
    }

    B::dtrace_print_Bf1_arr(&v, format!("{}{}", prefix, ".f1"));
    B::dtrace_print_Bf2_arr(&v, format!("{}{}", prefix, ".f2"));
    C::dtrace_print_fields_vec(&ref_arr, format!("{}{}", prefix, ".f3"));

    /* old implementation */
    // B::dtrace_print_Bf3_Cf1_arr(&v, format!("{}{}", prefix, ".f3.f1"));
    // B::dtrace_print_Bf3_Cf2_arr(&v, format!("{}{}", prefix, ".f3.f2"));
  }

  pub fn dtrace_print_pointer_arr(v: &[&B], prefix: String) {
    let mut traces = match File::options().append(true).open("main.dtrace") {
      Err(why) => panic!("Daikon couldn't open file, {}", why),
      Ok(traces) => traces,
    };
    writeln!(&mut traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
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
    writeln!(&mut traces, "{}", arr);
    writeln!(&mut traces, "0");
  }

  pub fn dtrace_print_Bf1_arr(v: &[&B], prefix: String) {
    let mut traces = match File::options().append(true).open("main.dtrace") {
      Err(why) => panic!("Daikon couldn't open file, {}", why),
      Ok(traces) => traces,
    };
    writeln!(&mut traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
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
    writeln!(&mut traces, "{}", arr);
    writeln!(&mut traces, "0");
  }

  pub fn dtrace_print_Bf2_arr(v: &[&B], prefix: String) {
    let mut traces = match File::options().append(true).open("main.dtrace") {
      Err(why) => panic!("Daikon couldn't open file, {}", why),
      Ok(traces) => traces,
    };
    writeln!(&mut traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
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
    writeln!(&mut traces, "{}", arr);
    writeln!(&mut traces, "0");
  }

  pub fn dtrace_print_Bf3_Cf1_arr(v: &[&B], prefix: String) {
    let mut traces = match File::options().append(true).open("main.dtrace") {
      Err(why) => panic!("Daikon couldn't open file, {}", why),
      Ok(traces) => traces,
    };
    writeln!(&mut traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
    let mut arr = String::from("[");
    let mut i = 0;
    while i < v.len()-1 {
      arr.push_str(&format!("{} ", v[i].f3.f1));
      i += 1;
    }
    if v.len() > 0 {
      arr.push_str(&format!("{}", v[v.len()-1].f3.f1));
    }
    arr.push_str("]");
    writeln!(&mut traces, "{}", arr);
    writeln!(&mut traces, "0");
  }

  pub fn dtrace_print_Bf3_Cf2_arr(v: &[&B], prefix: String) {
    let mut traces = match File::options().append(true).open("main.dtrace") {
      Err(why) => panic!("Daikon couldn't open file, {}", why),
      Ok(traces) => traces,
    };
    writeln!(&mut traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
    let mut arr = String::from("[");
    let mut i = 0;
    while i < v.len()-1 {
      arr.push_str(&format!("{} ", v[i].f3.f2));
      i += 1;
    }
    if v.len() > 0 {
      arr.push_str(&format!("{}", v[v.len()-1].f3.f2));
    }
    arr.push_str("]");
    writeln!(&mut traces, "{}", arr);
    writeln!(&mut traces, "0");
  }

} // impl B


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
  if v.len() > 0 {
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

fn dtrace_entry_no_nonce(name: &str) {
  let mut traces = match File::options().append(true).open("main.dtrace") {
    Err(why) => panic!("Daikon couldn't open file, {}", why),
    Ok(traces) => traces,
  };
  writeln!(&mut traces, "{}", name);
}

fn dtrace_exit_no_nonce(name: &str) {
  let mut traces = match File::options().append(true).open("main.dtrace") {
    Err(why) => panic!("Daikon couldn't open file, {}", why),
    Ok(traces) => traces,
  };
  writeln!(&mut traces, "{}", name);
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
