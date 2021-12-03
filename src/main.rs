#![allow(dead_code)]
#![feature(test)]

extern crate test;

mod utils;
mod day1;
mod day2;
mod day3;

fn main() {
    println!("{}", day3::fast::solution1());
}


