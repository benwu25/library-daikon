// synthesized imports
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};

mod a;
mod b;
mod c;
mod d;
mod container;
mod dummies;

use a::A;
use b::B;
use c::C;
use d::D;
use container::Container;
use dummies::{X,Y,Z};

fn main() {

  dtrace_entry_no_nonce("main:::ENTER");
  /* vars */
  dtrace_newline();

  let c = C { cf1: 12, cf2: 14 };
  let b = B { bf1: 23, bf2: 97, bf3: &c };
  let a = A { af1: "hullo", af2: 16, af3: &b };
  foo(&a, 17);

  let arr = [1, 2, 3];
  bar(&arr);

  let c2 = C { cf1: 19, cf2: 10 };
  let c3 = C { cf1: 7, cf2: 61 };
  let c_arr = [&c, &c2, &c3];
  baz(&c_arr);

  let b2 = B { bf1: 6, bf2: 1, bf3: &c2 };
  let b3 = B { bf1: 12, bf2: 11, bf3: &c3 };
  let b_arr = [&b, &b2, &b3];
  foo_bar(&b_arr);

  let d = D { df1: &arr };
  foo_baz(&d);

  let d2 = D { df1: &arr };
  let d3 = D { df1: &arr };
  let d_arr = [&d, &d2, &d3];
  bar_foo(&d_arr);

  let x1 = X { xf1: 22 };
  let x2 = X { xf1: 21 };
  let y1 = Y { yf1: 23 };
  let y2 = Y { yf1: 39 };
  let z1 = Z { zf1: 32 };
  let z2 = Z { zf1: 19 };
  let cont1 = Container { contf1: &x1, contf2: &y1, contf3: &z1 };
  let cont2 = Container { contf1: &x2, contf2: &y2, contf3: &z2 };
  let cont_arr = vec![&cont1, &cont2];
  bar_bar(&cont_arr);

  let a2 = A { af1: "a2s", af2: 91, af3: &b2 };
  let a3 = A { af1: "a3s", af2: 76, af3: &b3 };
  let a_vec = vec![&a, &a2, &a3];
  a_vec_depth3(&a_vec);
  a_vec_depth2(&a_vec);
  a_vec_depth1(&a_vec);

  dtrace_exit_no_nonce("main:::EXIT1");
  /* vars */
  dtrace_newline();

}

fn foo(a: &A, b: i32) {
  dtrace_entry("foo:::ENTER", *FOO_COUNTER.lock().unwrap());
  dtrace_print_pointer(a as *const _ as usize, String::from("a"));
  a.dtrace_print_fields(3, String::from("a"));
  dtrace_print_prim::<i32>(b, String::from("b"));
  dtrace_newline();


  dtrace_exit("foo:::EXIT1", *FOO_COUNTER.lock().unwrap());
  dtrace_print_pointer(a as *const _ as usize, String::from("a"));
  a.dtrace_print_fields(3, String::from("a"));
  dtrace_print_prim::<i32>(b, String::from("b"));
  dtrace_newline();
  *FOO_COUNTER.lock().unwrap() += 1;
}

fn bar(x: &[i32]) {
  dtrace_entry("bar:::ENTER", *BAR_COUNTER.lock().unwrap());
  dtrace_print_pointer(x as *const _ as *const () as usize, String::from("x"));
  dtrace_print_prim_arr::<i32>(x, String::from("x"));
  dtrace_newline();

  dtrace_exit("bar:::EXIT1", *BAR_COUNTER.lock().unwrap());
  dtrace_print_pointer(x as *const _ as *const () as usize, String::from("x"));
  dtrace_print_prim_arr::<i32>(x, String::from("x"));
  dtrace_newline();
  *BAR_COUNTER.lock().unwrap() += 1;
}

fn baz(c_arr: &[&C]) {
  dtrace_entry("baz:::ENTER", *BAZ_COUNTER.lock().unwrap());
  dtrace_print_pointer(c_arr as *const _ as *const () as usize, String::from("c_arr"));
  dtrace_print_pointer_arr::<C>(c_arr, format!("{}{}", String::from("c_arr"), "[..]"));
  C::dtrace_print_fields_arr(c_arr, 3, format!("{}{}", String::from("c_arr"), "[..]"));
  dtrace_newline();

  dtrace_exit("baz:::EXIT1", *BAZ_COUNTER.lock().unwrap());
  dtrace_print_pointer(c_arr as *const _ as *const () as usize, String::from("c_arr"));
  dtrace_print_pointer_arr::<C>(c_arr, format!("{}{}", String::from("c_arr"), "[..]"));
  C::dtrace_print_fields_arr(c_arr, 3, format!("{}{}", String::from("c_arr"), "[..]"));
  dtrace_newline();
  *BAZ_COUNTER.lock().unwrap() += 1;
}

fn foo_bar(b_arr: &[&B]) {
  dtrace_entry("foo_bar:::ENTER", *FOO_BAR_COUNTER.lock().unwrap());
  dtrace_print_pointer(b_arr as *const _ as *const () as usize, String::from("b_arr"));
  dtrace_print_pointer_arr::<B>(b_arr, format!("{}{}", String::from("b_arr"), "[..]"));
  B::dtrace_print_fields_arr(b_arr, 3, format!("{}{}", String::from("b_arr"), "[..]"));
  dtrace_newline();

  dtrace_exit("foo_bar:::EXIT1", *FOO_BAR_COUNTER.lock().unwrap());
  dtrace_print_pointer(b_arr as *const _ as *const () as usize, String::from("b_arr"));
  dtrace_print_pointer_arr::<B>(b_arr, format!("{}{}", String::from("b_arr"), "[..]"));
  B::dtrace_print_fields_arr(b_arr, 3, format!("{}{}", String::from("b_arr"), "[..]"));
  dtrace_newline();
  *FOO_BAR_COUNTER.lock().unwrap() += 1;
}

fn foo_baz(d: &D) {
  dtrace_entry("foo_baz:::ENTER", *FOO_BAZ_COUNTER.lock().unwrap());
  dtrace_print_pointer(d as *const _ as usize, String::from("d"));
  d.dtrace_print_fields(3, String::from("d"));
  dtrace_newline();

  dtrace_exit("foo_baz:::EXIT1", *FOO_BAZ_COUNTER.lock().unwrap());
  dtrace_print_pointer(d as *const _ as usize, String::from("d"));
  d.dtrace_print_fields(3, String::from("d"));
  dtrace_newline();
  *FOO_BAZ_COUNTER.lock().unwrap() += 1;
}

fn bar_foo(d_arr: &[&D]) {
  dtrace_entry("bar_foo:::ENTER", *BAR_FOO_COUNTER.lock().unwrap());
  dtrace_print_pointer(d_arr as *const _ as *const () as usize, String::from("d_arr"));
  dtrace_print_pointer_arr::<D>(d_arr, format!("{}{}", String::from("d_arr"), "[..]"));
  D::dtrace_print_fields_arr(d_arr, 3, format!("{}{}", String::from("d_arr"), "[..]"));
  dtrace_newline();

  dtrace_exit("bar_foo:::EXIT1", *BAR_FOO_COUNTER.lock().unwrap());
  dtrace_print_pointer(d_arr as *const _ as *const () as usize, String::from("d_arr"));
  dtrace_print_pointer_arr::<D>(d_arr, format!("{}{}", String::from("d_arr"), "[..]"));
  D::dtrace_print_fields_arr(d_arr, 3, format!("{}{}", String::from("d_arr"), "[..]"));
  dtrace_newline();
  *BAR_FOO_COUNTER.lock().unwrap() += 1;
}

fn bar_bar(cont_arr: &Vec<&Container>) {
  dtrace_entry("bar_bar:::ENTER", *BAR_BAR_COUNTER.lock().unwrap());
  dtrace_print_pointer(cont_arr.as_ptr() as usize, String::from("cont_arr"));
  dtrace_print_pointer_vec::<Container>(cont_arr, format!("{}{}", String::from("cont_arr"), "[..]"));
  Container::dtrace_print_fields_vec(cont_arr, 3, format!("{}{}", String::from("cont_arr"), "[..]"));
  dtrace_newline();

  dtrace_exit("bar_bar:::EXIT1", *BAR_BAR_COUNTER.lock().unwrap());
  dtrace_print_pointer(cont_arr.as_ptr() as usize, String::from("cont_arr"));
  dtrace_print_pointer_vec::<Container>(cont_arr, format!("{}{}", String::from("cont_arr"), "[..]"));
  Container::dtrace_print_fields_vec(cont_arr, 3, format!("{}{}", String::from("cont_arr"), "[..]"));
  dtrace_newline();
  *BAR_BAR_COUNTER.lock().unwrap() += 1;
}

fn a_vec_depth3(a_vec: &Vec<&A>) {
  dtrace_entry("a_vec_depth3:::ENTER", *A_VEC_DEPTH3_COUNTER.lock().unwrap());
  dtrace_print_pointer(a_vec.as_ptr() as usize, String::from("a_vec"));
  dtrace_print_pointer_vec::<A>(a_vec, format!("{}{}", String::from("a_vec"), "[..]"));
  A::dtrace_print_fields_vec(a_vec, 3, format!("{}{}", String::from("a_vec"), "[..]"));
  dtrace_newline();

  dtrace_exit("a_vec_depth3:::EXIT1", *A_VEC_DEPTH3_COUNTER.lock().unwrap());
  dtrace_print_pointer(a_vec.as_ptr() as usize, String::from("a_vec"));
  dtrace_print_pointer_vec::<A>(a_vec, format!("{}{}", String::from("a_vec"), "[..]"));
  A::dtrace_print_fields_vec(a_vec, 3, format!("{}{}", String::from("a_vec"), "[..]"));
  dtrace_newline();
  *A_VEC_DEPTH3_COUNTER.lock().unwrap() += 1;
}

fn a_vec_depth2(a_vec: &Vec<&A>) {
  dtrace_entry("a_vec_depth2:::ENTER", *A_VEC_DEPTH2_COUNTER.lock().unwrap());
  dtrace_print_pointer(a_vec.as_ptr() as usize, String::from("a_vec"));
  dtrace_print_pointer_vec::<A>(a_vec, format!("{}{}", String::from("a_vec"), "[..]"));
  A::dtrace_print_fields_vec(a_vec, 2, format!("{}{}", String::from("a_vec"), "[..]"));  
  dtrace_newline();

  dtrace_exit("a_vec_depth2:::EXIT1", *A_VEC_DEPTH2_COUNTER.lock().unwrap());
  dtrace_print_pointer(a_vec.as_ptr() as usize, String::from("a_vec"));
  dtrace_print_pointer_vec::<A>(a_vec, format!("{}{}", String::from("a_vec"), "[..]"));
  A::dtrace_print_fields_vec(a_vec, 2, format!("{}{}", String::from("a_vec"), "[..]"));
  dtrace_newline();
  *A_VEC_DEPTH2_COUNTER.lock().unwrap() += 1;
}

fn a_vec_depth1(a_vec: &Vec<&A>) {
  dtrace_entry("a_vec_depth1:::ENTER", *A_VEC_DEPTH1_COUNTER.lock().unwrap());
  dtrace_print_pointer(a_vec.as_ptr() as usize, String::from("a_vec"));
  dtrace_print_pointer_vec::<A>(a_vec, format!("{}{}", String::from("a_vec"), "[..]"));
  A::dtrace_print_fields_vec(a_vec, 1, format!("{}{}", String::from("a_vec"), "[..]"));
  dtrace_newline();

  dtrace_exit("a_vec_depth1:::EXIT1", *A_VEC_DEPTH1_COUNTER.lock().unwrap());
  dtrace_print_pointer(a_vec.as_ptr() as usize, String::from("a_vec"));
  dtrace_print_pointer_vec::<A>(a_vec, format!("{}{}", String::from("a_vec"), "[..]"));
  A::dtrace_print_fields_vec(a_vec, 1, format!("{}{}", String::from("a_vec"), "[..]"));
  dtrace_newline();
  *A_VEC_DEPTH1_COUNTER.lock().unwrap() += 1;
}



//== private daikon library ==//

// dtrace File
static tr: LazyLock<Mutex<Option<File>>> = LazyLock::new(||
                                               Mutex::new(dtrace_open()));

// nonce counters
static FOO_COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static BAR_COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static BAZ_COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static FOO_BAR_COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static FOO_BAZ_COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static BAR_FOO_COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static BAR_BAR_COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static A_VEC_DEPTH3_COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static A_VEC_DEPTH2_COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static A_VEC_DEPTH1_COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));

fn dtrace_open() -> Option<File> {
  match File::options().append(true).open("main.dtrace") {
    Err(why) => {
      panic!("Daikon couldn't open file, {}", why);
    },
    Ok(traces) => Some(traces),
  }
}

// alternative syntax for dtrace routines.
pub fn dtrace_print_pointer_arr<T>(v: &[&T], var_name: String) {
  if let Some(traces) = &mut *tr.lock().unwrap() {
    writeln!(traces, "{}", var_name).ok();
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
    writeln!(traces, "{}", arr).ok();
    writeln!(traces, "0").ok();
  } else {
    panic!("dtrace file is not open");
  }
}

pub fn dtrace_print_pointer_vec<T>(v: &Vec<&T>, var_name: String) {
  match &mut *tr.lock().unwrap() {
    None => panic!("dtrace file is not open"),
    Some(traces) => {
      writeln!(traces, "{}", var_name).ok();
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
      writeln!(traces, "{}", arr).ok();
      writeln!(traces, "0").ok();
    },
  }
}

// T must implement Display trait
fn dtrace_print_prim_arr<T: std::fmt::Display>(v: &[T], prefix: String) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", format!("{}{}", prefix, "[..]")).ok();
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
      writeln!(traces, "{}", arr).ok();
      writeln!(traces, "0").ok();
    },
  };
}

fn dtrace_print_str(v: &str, var_name: String) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", var_name).ok();
      writeln!(traces, "{}", v).ok();
      writeln!(traces, "0").ok();    
    }
  }
}

// T must implement Display trait
fn dtrace_print_prim<T: std::fmt::Display>(v: T, var_name: String) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", var_name).ok();
      writeln!(traces, "{}", v).ok();
      writeln!(traces, "0").ok();    
    }
  }
}

fn dtrace_print_pointer(v: usize, var_name: String) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", var_name).ok();
      writeln!(traces, "0x{:x}", v).ok();
      writeln!(traces, "0").ok();
    },
  };
}

fn dtrace_entry_no_nonce(name: &str) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", name).ok();
    }
  }
}

fn dtrace_exit_no_nonce(name: &str) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", name).ok();
    }
  }
}

fn dtrace_entry(name: &str, nonce: u32) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", name).ok();
      writeln!(traces, "this_invocation_nonce").ok();
      writeln!(traces, "{}", nonce).ok();
    },
  }
}

fn dtrace_exit(name: &str, nonce: u32) {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "{}", name).ok();
      writeln!(traces, "this_invocation_nonce").ok();
      writeln!(traces, "{}", nonce).ok();
    },
  }
}

fn dtrace_newline() {
  match &mut *tr.lock().unwrap() {
    None => { panic!("dtrace file is not open"); },
    Some(traces) => {
      writeln!(traces, "").ok();
    }
  }
}
