
#![feature(test)]

extern crate test;
extern crate regex;
extern crate rand;

use rand::{thread_rng, Rng, ThreadRng};
use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::iter::Iterator;


// Random topic string generator
pub struct Topics {
	choices : Vec<String>,
    rng : ThreadRng,
}

impl Topics {
	pub fn new(file : &str) -> Topics {
		let f = File::open(file).unwrap();
		let file = BufReader::new(&f);
		let mut choices : Vec<String> = file.lines().map(|l| l.unwrap().split_whitespace().last().unwrap().to_string()).collect();
		let mut wilds : Vec<String> = std::iter::repeat(".+".to_string()).take(300).collect(); 
		choices.append(&mut wilds);
    	let rng = thread_rng();
		Topics { choices, rng }	
	}
}

impl Iterator for Topics {
	type Item = String;
	fn next(&mut self) -> Option<Self::Item> {
        let pathlen = ((self.rng.next_f64() * 7.0) + 2.0) as usize;
        let mut segments : Vec<String> = (0..pathlen).map(|_| self.rng.choose(&self.choices).unwrap().to_string() ).collect();
        if self.rng.next_f64() > 0.8 {
            segments.push(".*".to_string());
        }
        Some(segments.join("/"))
	}
}

fn main() {

	let topics = Topics::new("eff_short_wordlist_2_0.txt");
	
	for i in topics.take(100) {
		println!("{}", i);
	}
}

#[bench]
fn create_ten_thousand(b: &mut test::Bencher) {
	let topics = Topics::new("eff_short_wordlist_2_0.txt");
	let paths : Vec<String> = topics.take(10_000).collect();
    b.iter(move || {
    	let set = regex::RegexSet::new(&paths).unwrap();
        test::black_box(set);
    });
}
