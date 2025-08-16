// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};

pub struct C {
  pub f1: i16,
  pub f2: i32,
  // pub f3: array or vec
}

impl C {
  pub fn dtrace_print(&self, depth: i32, prefix: String) {
    dtrace_print_pointer(self as *const _ as usize, prefix.clone());
    if depth == 0 {
      return;
    }
    dtrace_print_prim::<i16>(self.f1, format!("{}{}", prefix, ".f1"));
    dtrace_print_prim::<i32>(self.f2, format!("{}{}", prefix, ".f2"));
  }

  // don't care about addr of v or addr of C's, this is only called when
  // C's are nested in an array of some other struct like B
  pub fn dtrace_print_fields_vec(v: &Vec<&C>, prefix: String) {
    // dtrace_print_pointer(v[0] as *const _ as usize, prefix.clone());

    // C::dtrace_print_pointer_vec(v, prefix.clone());

    C::dtrace_print_f1_vec(v, format!("{}{}", prefix, ".f1"));
    C::dtrace_print_f2_vec(v, format!("{}{}", prefix, ".f2"));
  }

  pub fn dtrace_print_arr(v: &[&C], prefix: String) {
    dtrace_print_pointer(v as *const _ as *const () as usize, prefix.clone());

    // generic routine possible for ptr array?
    C::dtrace_print_pointer_arr(&v, prefix.clone());


    // do a dfs traversal of all fields and subfields of C, if field f is a
    // primitive (leaf node), add a call to print the array trace record for
    // that primitive with the particular access pattern for that field. These
    // for loops can be build from string fragments which have gaps where the
    // accessor strings to go, see example break up fragments* below. Build
    // the accessors during the traversal.

    // C::f1
    C::dtrace_print_f1_arr(&v, format!("{}{}", prefix, ".f1"));

    // C::f2
    C::dtrace_print_f2_arr(&v, format!("{}{}", prefix, ".f2"));
  }

  // TODO
  pub fn dtrace_print_pointer_vec(v: &Vec<&C>, prefix: String) {
    let mut traces = match File::options().append(true).open("main.dtrace") {
      Err(why) => panic!("Daikon couldn't open file, {}", why),
      Ok(traces) => traces,
    };
    writeln!(&mut traces, "{}", format!("{}{}", prefix.clone(), "[..]"));
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
    writeln!(&mut traces, "{}", arr);
    writeln!(&mut traces, "0");
  }

  pub fn dtrace_print_pointer_arr(v: &[&C], prefix: String) {
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

  // TODO
  pub fn dtrace_print_f1_vec(v: &Vec<&C>, prefix: String) {
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
      arr.push_str(&format!("{}", v[v.len() - 1].f1));
    }
    arr.push_str("]");
    writeln!(&mut traces, "{}", arr);
    writeln!(&mut traces, "0");
  }

  // like dtrace_print_prim_arr but v[i].f1 instead of v[i]
  pub fn dtrace_print_f1_arr(v: &[&C], prefix: String) {
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

  // TODO
  pub fn dtrace_print_f2_vec(v: &Vec<&C>, prefix: String) {
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

  // v[i].f2 instead of v[i]
  pub fn dtrace_print_f2_arr(v: &[&C], prefix: String) {
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
