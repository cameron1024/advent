#![allow(dead_code)]
#![feature(test)]


extern crate test;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod utils;

fn main() {
    dbg!(day13::solution1());
    day13::print_solution2();
}
