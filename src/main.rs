#![allow(dead_code)]
use rayon::prelude::*;
use std::{thread, time::Duration};

fn main() {
    let l = vec![43, 54, 65, 76, 8, 7, 65, 34];
    let x: i32 = l.par_iter().map(|i| i * i).sum();
    println!("{}", x);
    let x: i32 = l.par_iter().map(|i| calc(i)).sum();
    println!("{}", x);
}

fn calc(x: &i32) -> i32 {
    thread::sleep(Duration::new(0, 100e0));
    println!("{}", x * x);
    x * x
}
