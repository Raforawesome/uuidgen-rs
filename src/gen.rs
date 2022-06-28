#![allow(unused, dead_code)]
use std::{sync::Mutex, ops::Index};
use rand::prelude::*;

const LENGTH: i32 = 16;

const CHARS: &str = "abcdefghijklmnopqrstuvwxyz1234567890";
static USED: Mutex<Vec<String>> = Mutex::new(vec![]);


fn random_num(rng: &mut ThreadRng, min: i32, mut max: i32) -> i32 {
	max += 1;
	let alpha: f32 = rand::random::<f32>();
	min + ((max - min) as f32 * alpha) as i32
}

fn random_char(rng: &mut ThreadRng) -> char {
	let idx: usize = random_num(rng, 0, CHARS.len() as i32) as usize;
	CHARS.bytes().collect::<Vec<u8>>()[idx] as char
}

pub fn gen_uuid(dashed: bool) -> String {
	let mut rng: ThreadRng = rand::thread_rng();
	if dashed {
		let mut s: String = String::new();
		for i in 1..=4 {
			for _ in 0..4 {
				s.push(random_char(&mut rng));
			}
			if i != 4 {
				s.push('-')
			}
		}
		s
	} else {
		let mut s: String = String::new();
		for _ in 0..LENGTH {
			s.push(random_char(&mut rng));
		}
		s
	}
}

fn add_uuid(s: String) {
	// Using .unwrap() because:
	// I assume this same thread will never have the mutex elsewhere
	// And that this code will never panic while using this mutex
	let mut guard = USED.lock().unwrap();
}