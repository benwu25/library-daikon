// synthesized imports
use crate::tr;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};

pub struct C {
    pub cf1: i16,
    pub cf2: i32,
}

impl C {
    pub fn dtrace_print_fields(&self, depth: i32, prefix: String) {
        if depth == 0 {
            return;
        }
        dtrace_print_prim::<i16>(self.cf1, format!("{}{}", prefix, ".cf1"));
        dtrace_print_prim::<i32>(self.cf2, format!("{}{}", prefix, ".cf2"));
    }

    pub fn dtrace_print_fields_vec(v: &Vec<&C>, depth: i32, prefix: String) {
        if depth == 0 {
            return;
        }

        C::dtrace_print_cf1_vec(v, format!("{}{}", prefix, ".cf1"));
        C::dtrace_print_cf2_vec(v, format!("{}{}", prefix, ".cf2"));
    }

    pub fn dtrace_print_fields_arr(v: &[&C], depth: i32, prefix: String) {
        if depth == 0 {
            return;
        }

        C::dtrace_print_cf1_arr(v, format!("{}{}", prefix, ".cf1"));
        C::dtrace_print_cf2_arr(v, format!("{}{}", prefix, ".cf2"));
    }

    pub fn dtrace_print_pointer_vec(v: &Vec<&C>, prefix: String) {
        match &mut *tr.lock().unwrap() {
            None => panic!("dtrace file is not open"),
            Some(traces) => {
                writeln!(traces, "{}", format!("{}{}", prefix, "[..]")).ok();
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
                writeln!(traces, "{}", arr).ok();
                writeln!(traces, "0").ok();
            }
        }
    }

    pub fn dtrace_print_cf1_vec(v: &Vec<&C>, var_name: String) {
        match &mut *tr.lock().unwrap() {
            None => panic!("dtrace file is not open"),
            Some(traces) => {
                writeln!(traces, "{}", var_name).ok();
                let mut arr = String::from("[");
                let mut i = 0;
                while i < v.len() - 1 {
                    arr.push_str(&format!("{} ", v[i].cf1));
                    i += 1;
                }
                if v.len() > 0 {
                    arr.push_str(&format!("{}", v[v.len() - 1].cf1));
                }
                arr.push_str("]");
                writeln!(traces, "{}", arr).ok();
                writeln!(traces, "0").ok();
            }
        }
    }

    // like dtrace_print_prim_arr but v[i].f1 instead of v[i]
    pub fn dtrace_print_cf1_arr(v: &[&C], var_name: String) {
        match &mut *tr.lock().unwrap() {
            None => panic!("dtrace file is not open"),
            Some(traces) => {
                writeln!(traces, "{}", var_name).ok();
                let mut arr = String::from("[");
                let mut i = 0;
                while i < v.len() - 1 {
                    arr.push_str(&format!("{} ", v[i].cf1));
                    i += 1;
                }
                if v.len() > 0 {
                    arr.push_str(&format!("{}", v[v.len() - 1].cf1));
                }
                arr.push_str("]");
                writeln!(traces, "{}", arr).ok();
                writeln!(traces, "0").ok();
            }
        }
    }

    pub fn dtrace_print_cf2_vec(v: &Vec<&C>, var_name: String) {
        match &mut *tr.lock().unwrap() {
            None => panic!("dtrace file is not open"),
            Some(traces) => {
                writeln!(traces, "{}", var_name).ok();
                let mut arr = String::from("[");
                let mut i = 0;
                while i < v.len() - 1 {
                    arr.push_str(&format!("{} ", v[i].cf2));
                    i += 1;
                }
                if v.len() > 0 {
                    arr.push_str(&format!("{}", v[v.len() - 1].cf2));
                }
                arr.push_str("]");
                writeln!(traces, "{}", arr).ok();
                writeln!(traces, "0").ok();
            }
        }
    }

    pub fn dtrace_print_cf2_arr(v: &[&C], var_name: String) {
        match &mut *tr.lock().unwrap() {
            None => panic!("dtrace file is not open"),
            Some(traces) => {
                writeln!(traces, "{}", var_name).ok();
                let mut arr = String::from("[");
                let mut i = 0;
                while i < v.len() - 1 {
                    arr.push_str(&format!("{} ", v[i].cf2));
                    i += 1;
                }
                if v.len() > 0 {
                    arr.push_str(&format!("{}", v[v.len() - 1].cf2));
                }
                arr.push_str("]");
                writeln!(traces, "{}", arr).ok();
                writeln!(traces, "0").ok();
            }
        }
    }
}

//== daikon private library ==//

// nonce counters
/* none */

pub fn dtrace_print_pointer_arr<T>(v: &[&T], var_name: String) {
    match &mut *tr.lock().unwrap() {
        None => panic!("dtrace file is not open"),
        Some(traces) => {
            writeln!(traces, "{}", var_name).ok();
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
            writeln!(traces, "{}", arr).ok();
            writeln!(traces, "0").ok();
        }
    }
}

pub fn dtrace_print_pointer_vec<T>(v: &Vec<&T>, var_name: String) {
    match &mut *tr.lock().unwrap() {
        None => panic!("dtrace file is not open"),
        Some(traces) => {
            writeln!(traces, "{}", var_name).ok();
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
            writeln!(traces, "{}", arr).ok();
            writeln!(traces, "0").ok();
        }
    }
}

// T must implement Display trait
fn dtrace_print_prim_arr<T: std::fmt::Display>(v: &[T], prefix: String) {
    dtrace_print_pointer(v as *const _ as *const () as usize, prefix.clone());
    match &mut *tr.lock().unwrap() {
        None => panic!("dtrace file is not open"),
        Some(traces) => {
            writeln!(traces, "{}", format!("{}{}", prefix, "[..]")).ok();
            let mut arr = String::from("[");
            let mut i = 0;
            while i < v.len() - 1 {
                arr.push_str(&format!("{} ", v[i]));
                i += 1;
            }
            if v.len() > 0 {
                arr.push_str(&format!("{}", v[v.len() - 1]));
            }
            arr.push_str("]");
            writeln!(traces, "{}", arr).ok();
            writeln!(traces, "0").ok();
        }
    }
}

fn dtrace_print_str(v: &str, var_name: String) {
    match &mut *tr.lock().unwrap() {
        None => panic!("dtrace file is not open"),
        Some(traces) => {
            writeln!(traces, "{}", var_name).ok();
            writeln!(traces, "\"{}\"", v).ok();
            writeln!(traces, "0").ok();
        }
    }
}

// T must implement Display trait
fn dtrace_print_prim<T: std::fmt::Display>(v: T, var_name: String) {
    match &mut *tr.lock().unwrap() {
        None => panic!("dtrace file is not open"),
        Some(traces) => {
            writeln!(traces, "{}", var_name).ok();
            writeln!(traces, "{}", v).ok();
            writeln!(traces, "0").ok();
        }
    }
}

fn dtrace_print_pointer(v: usize, var_name: String) {
    match &mut *tr.lock().unwrap() {
        None => panic!("dtrace file is not open"),
        Some(traces) => {
            writeln!(traces, "{}", var_name).ok();
            writeln!(traces, "0x{:x}", v).ok();
            writeln!(traces, "0").ok();
        }
    }
}

fn dtrace_entry_no_nonce(ppt_name: &str) {
    match &mut *tr.lock().unwrap() {
        None => panic!("dtrace file is not open"),
        Some(traces) => {
            writeln!(traces, "{}", ppt_name).ok();
        }
    }
}

fn dtrace_exit_no_nonce(ppt_name: &str) {
    match &mut *tr.lock().unwrap() {
        None => panic!("dtrace file is not open"),
        Some(traces) => {
            writeln!(traces, "{}", ppt_name).ok();
        }
    }
}

fn dtrace_entry(ppt_name: &str, nonce: u32) {
    match &mut *tr.lock().unwrap() {
        None => panic!("dtrace file is not open"),
        Some(traces) => {
            writeln!(traces, "{}", ppt_name).ok();
            writeln!(traces, "this_invocation_nonce").ok();
            writeln!(traces, "{}", nonce).ok();
        }
    }
}

fn dtrace_exit(ppt_name: &str, nonce: u32) {
    match &mut *tr.lock().unwrap() {
        None => panic!("dtrace file is not open"),
        Some(traces) => {
            writeln!(traces, "{}", ppt_name).ok();
            writeln!(traces, "this_invocation_nonce").ok();
            writeln!(traces, "{}", nonce).ok();
        }
    }
}

fn dtrace_newline() {
    match &mut *tr.lock().unwrap() {
        None => panic!("dtrace file is not open"),
        Some(traces) => {
            writeln!(traces, "").ok();
        }
    }
}
