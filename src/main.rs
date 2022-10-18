#![allow(unused)]

use std::io;
use rand::{Rng, random};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let today: Day = Day::Sunday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        _  => println!("Everybody like this day.")
    }
    println!("Is today the weekend? {}" , today.is_weekend() );
}
