#![allow(unused, dead_code)]
use std::sync::Mutex;

static USED: Mutex<Vec<String>> = Mutex::new(vec![]);