#![allow(unused, dead_code)]
use std::{sync::Mutex, ops::Index};
use rand::prelude::*;

const CHARS: &str = "abcdefghijklmnopqrstuvwxyz1234567890";
static USED: Mutex<Vec<String>> = Mutex::new(vec![]);


fn random_num(rng: &ThreadRng, min: i32, mut max: i32) -> i32 {
	max += 1;
	let alpha: f32 = rand::random::<f32>();
	min + ((max - min) as f32 * alpha) as i32
}

fn random_char(rng: &ThreadRng) -> char {
	let idx: usize = random_num(rng, 0, CHARS.len() as i32) as usize;
	CHARS.bytes().collect::<Vec<u8>>()[idx] as char
}