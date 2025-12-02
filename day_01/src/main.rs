/*
https://adventofcode.com/2025/day/1
--- Day 1: Secret Entrance ---
 */
use std::io;
use std::io::prelude::*;
use aoc::args::is_debug;

fn main() {
    let mut lines = io::stdin().lock().lines();

    let mut previous_pointed_value;
    let mut pointed_value = 50;
    let mut count_pointed_zero = 0; // part 1
    let mut count_clicks_zero = 0; // part 2

    let dbg = is_debug();

    while let Some(Ok(line)) = lines.next() {
        if line.len() == 0 {
            break;
        }
        let mut chs = line.chars();
        let direction: char = chs.next().unwrap(); // pop first char
        let displace: i32 = chs.as_str().parse::<i32>().unwrap();
        previous_pointed_value = pointed_value;
        match direction {
            'L' => { pointed_value -= displace }
            'R' => { pointed_value += displace }
            _ => { panic!("bad input format") }
        }
        // Handle negative modulos because once again this language
        // provides a REMAINDER possibly negative and not the
        // rational choice of staying positive modulo...
        if pointed_value >= 0 {
            pointed_value = pointed_value % 100;
        } else {
            pointed_value = (pointed_value % 100 + 100) % 100;
        }
        if pointed_value == 0 {
            count_pointed_zero += 1;
        }

        // For Part 2: lot of care about fencepost error, to not
        // count twice or 0 the steps that start or stop exactly at 0.

        // 1 full rotation + some remainder, count implicit clicks over zero
        if displace > 100 {
            let rotations = displace / 100;
            count_clicks_zero += rotations;
            if dbg {
                println!("at {line}: group of +{rotations} zeros");
            }
        }
        // count if the last partial rotation was over zero once.
        let mod_displace = displace%100;
        if mod_displace != 0 {
            match direction {
                'L' => if previous_pointed_value > 0 && previous_pointed_value <= mod_displace  {
                    count_clicks_zero += 1;
                    if dbg {
                        println!("at {line}: +1 zeros from {previous_pointed_value} - {mod_displace}");
                    }
                }
                'R' => if previous_pointed_value + mod_displace >= 100  {
                    count_clicks_zero += 1;
                    if dbg {
                        println!("at {line}: +1 zeros from {previous_pointed_value} + {mod_displace}");
                    }
                }
                _ => {  }
            }
        }
    }

    
    println!("== Part 1: {count_pointed_zero}");
    println!("== Part 2: {count_clicks_zero}");

}
