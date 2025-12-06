/*
https://adventofcode.com/2025/day/3
--- Day 3: Lobby ---
 */
use aoc::args::is_debug;
use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;

/*
 * The max number of a constant number of digit, will
 * always be made by maximizing each digit in succession.
 */
fn max_joltage(bank: &Vec<u32>, depth: usize) -> u64 {
    let len = bank.len();

    let mut max: u64 = 0;
    // start looking for next digit at...
    let mut next_digit_idx = 0;
    let mut remaining_digits = depth;

    for _ in 0..depth {
        remaining_digits -= 1;
        // Find the biggest digit in the available range
        // (don't search in the end of the bank to let room
        // to find for the last digits of the final joltage)
        // For equally big numbers, choose the first ones
        // a tuple of (digit_position, digit_value)
        let max_digit: (usize, &u32) = bank[next_digit_idx..(len - remaining_digits)]
            .iter()
            .enumerate()
            .max_by(|(i1, v1), (i2, v2)| {
                if v1 < v2 {
                    Ordering::Less
                } else if v1 > v2 {
                    Ordering::Greater
                }
                // Ordering results for equal values
                // but different indices are choosen
                // so the earlier index is prefered !
                // else max_by() by default returns the latest element.
                else if i1 < i2 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .unwrap();
        max = max * 10 + (*max_digit.1 as u64);
        // max_digit.0 is the index of the iterator in the sliced bank[next_digit_idx..],
        // so it always start at 0 ! we need the absolute value from the unsliced bank
        next_digit_idx += max_digit.0 + 1;
    }

    max
}

fn main() {
    let dbg = is_debug();
    let mut banks = Vec::<Vec<u32>>::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let bank: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        banks.push(bank);
    }

    if dbg {
        println!("banks  = {:?}", banks);
    }

    let mut jolt1: u64 = 0;
    for bank in banks.iter() {
        let j = max_joltage(&bank, 2);
        jolt1 += j;
        if dbg {
            println!("Found joltage {j} in bank {:?}", bank);
        }
    }
    println!("== Part 1: {jolt1}");

    let mut jolt2: u64 = 0;
    for bank in banks.iter() {
        let j = max_joltage(&bank, 12);
        jolt2 += j;
        if dbg {
            println!("Found joltage(12) {j} in bank {:?}", bank);
        }
    }
    println!("== Part 2: {jolt2}");
}
