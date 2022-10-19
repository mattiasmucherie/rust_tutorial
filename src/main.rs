#![allow(unused)]

use std::io;
use rand::{Rng, random};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;

mod restaurant;
use restaurant::order_food;

fn main() {
    order_food();
}
