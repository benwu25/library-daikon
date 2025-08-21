// synthesized imports
use crate::tr;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{LazyLock, Mutex};

use crate::c::C;

pub struct B<'a> {
    pub bf1: i32,
    pub bf2: u16,
    pub bf3: &'a C,
}

impl<'a> B<'a> {
    pub fn dtrace_print_fields(&self, depth: i32, prefix: String) {
        if depth == 0 {
            return;
        }
        dtrace_print_prim::<i32>(self.bf1, format!("{}{}", prefix, ".bf1"));
        dtrace_print_prim::<u16>(self.bf2, format!("{}{}", prefix, ".bf2"));
        dtrace_print_pointer(
            self.bf3 as *const _ as usize,
            format!("{}{}", prefix, ".bf3"),
        );

        self.bf3
            .dtrace_print_fields(depth - 1, format!("{}{}", prefix, ".bf3"));
    }

    pub fn dtrace_print_fields_arr(v: &[&B], depth: i32, prefix: String) {
        if depth == 0 {
            return;
        }

        let mut c_ref_arr = Vec::new();
        let mut i = 0;
        while i < v.len() {
            c_ref_arr.push(v[i].bf3);
            i += 1;
        }

        B::dtrace_print_bf1_arr(v, format!("{}{}", prefix, ".bf1"));
        B::dtrace_print_bf2_arr(v, format!("{}{}", prefix, ".bf2"));
        dtrace_print_pointer_vec::<C>(&c_ref_arr, format!("{}{}", prefix, ".bf3"));
        C::dtrace_print_fields_vec(&c_ref_arr, depth - 1, format!("{}{}", prefix, ".bf3"));
    }

    pub fn dtrace_print_fields_vec(v: &Vec<&B>, depth: i32, prefix: String) {
        if depth == 0 {
            return;
        }

        let mut c_ref_arr = Vec::new();
        let mut i = 0;
        while i < v.len() {
            c_ref_arr.push(v[i].bf3);
            i += 1;
        }

        B::dtrace_print_bf1_vec(v, format!("{}{}", prefix, ".bf1"));
        B::dtrace_print_bf2_vec(v, format!("{}{}", prefix, ".bf2"));
        dtrace_print_pointer_vec::<C>(&c_ref_arr, format!("{}{}", prefix, ".bf3"));
        C::dtrace_print_fields_vec(&c_ref_arr, depth - 1, format!("{}{}", prefix, ".bf3"));
    }

    pub fn dtrace_print_bf1_vec(v: &Vec<&B>, var_name: String) {
        match &mut *tr.lock().unwrap() {
            None => panic!("dtrace file is not open"),
            Some(traces) => {
                writeln!(traces, "{}", var_name).ok();
                let mut arr = String::from("[");
                let mut i = 0;
                while i < v.len() - 1 {
                    arr.push_str(&format!("{} ", v[i].bf1));
                    i += 1;
                }
                if v.len() > 0 {
                    arr.push_str(&format!("{}", v[v.len() - 1].bf1));
                }
                arr.push_str("]");
                writeln!(traces, "{}", arr).ok();
                writeln!(traces, "0").ok();
            }
        }
    }

    pub fn dtrace_print_bf2_vec(v: &Vec<&B>, var_name: String) {
        match &mut *tr.lock().unwrap() {
            None => panic!("dtrace file is not open"),
            Some(traces) => {
                writeln!(traces, "{}", var_name).ok();
                let mut arr = String::from("[");
                let mut i = 0;
                while i < v.len() - 1 {
                    arr.push_str(&format!("{} ", v[i].bf2));
                    i += 1;
                }
                if v.len() > 0 {
                    arr.push_str(&format!("{}", v[v.len() - 1].bf2));
                }
                arr.push_str("]");
                writeln!(traces, "{}", arr).ok();
                writeln!(traces, "0").ok();
            }
        }
    }

    pub fn dtrace_print_bf1_arr(v: &[&B], var_name: String) {
        match &mut *tr.lock().unwrap() {
            None => panic!("dtrace file is not open"),
            Some(traces) => {
                writeln!(traces, "{}", var_name).ok();
                let mut arr = String::from("[");
                let mut i = 0;
                while i < v.len() - 1 {
                    arr.push_str(&format!("{} ", v[i].bf1));
                    i += 1;
                }
                if v.len() > 0 {
                    arr.push_str(&format!("{}", v[v.len() - 1].bf1));
                }
                arr.push_str("]");
                writeln!(traces, "{}", arr).ok();
                writeln!(traces, "0").ok();
            }
        }
    }

    pub fn dtrace_print_bf2_arr(v: &[&B], var_name: String) {
        match &mut *tr.lock().unwrap() {
            None => panic!("dtrace file is not open"),
            Some(traces) => {
                writeln!(traces, "{}", var_name).ok();
                let mut arr = String::from("[");
                let mut i = 0;
                while i < v.len() - 1 {
                    arr.push_str(&format!("{} ", v[i].bf2));
                    i += 1;
                }
                if v.len() > 0 {
                    arr.push_str(&format!("{}", v[v.len() - 1].bf2));
                }
                arr.push_str("]");
                writeln!(traces, "{}", arr).ok();
                writeln!(traces, "0").ok();
            }
        }
    }
} // impl B

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
