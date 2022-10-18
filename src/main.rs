#![allow(unused)]

use std::io;
use rand::{Rng, random};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,123,3,4];
    vec2.push(5);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No second value...")
    }


    for i in &mut vec2 {
        *i *= 2;
    };
    for i in &vec2 {
        println!("{}",i);
    };
}
