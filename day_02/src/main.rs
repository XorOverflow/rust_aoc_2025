/*
https://adventofcode.com/2025/day/2
--- Day 2: Gift Shop ---
 */
use aoc::args::is_debug;
use std::io;
use std::io::prelude::*;
use std::ops::RangeInclusive;

fn sum_invalid_ids(r: RangeInclusive<u64>) -> u64 {
    let dbg = is_debug();
    let mut invalid: u64 = 0;

    for k in r {
        let digits: Vec<char> = k.to_string().chars().collect();
        let dlen = digits.len();
        let half = dlen / 2;
        if half * 2 != dlen {
            // to repeat a pattern twice, the string must be even-length
            continue;
        } else {
            let half1 = &digits[..half];
            let half2 = &digits[half..];
            if half1 == half2 {
                if dbg {
                    println!("{k} is invalid !");
                }
                invalid += k;
            }
        }
    }
    invalid
}

fn sum_invalid_ids_any(r: RangeInclusive<u64>) -> u64 {
    let dbg = is_debug();
    let mut invalid: u64 = 0;

    'ids: for k in r {
        let digits: Vec<char> = k.to_string().chars().collect();
        let dlen = digits.len();

        'subsizes: for subsize in 1..dlen {
            // Not divisible exactly
            if dlen % subsize != 0 {
                continue 'subsizes;
            }

            let parts = dlen / subsize;

            let part_1 = &digits[..subsize];
            for p in 0..parts {
                let part_n = &digits[(subsize * p)..(subsize * (p + 1))];
                if part_1 != part_n {
                    continue 'subsizes;
                }
            }
            if dbg {
                println!("{k} is invalid with subsize {subsize} !");
            }
            invalid += k;
            // Don't test other subdivisions, which could count the
            // same ID several times
            continue 'ids;
        }
    }
    invalid
}

fn main() {
    let dbg = is_debug();
    // The input is if inclusive range (end is part of the range),
    // not the "Range" type which omits the last element.
    let pairs: Vec<RangeInclusive<u64>>;
    let mut lines = io::stdin().lock().lines();
    if let Some(Ok(input)) = lines.next() {
        let ranges = input.split(',');
        pairs = ranges
            .map(|s| {
                let s2 = s.split_once('-').unwrap();
                let r1 = s2.0.parse::<u64>().unwrap();
                let r2 = s2.1.parse::<u64>().unwrap();
                r1..=r2
            })
            .collect();
    } else {
        panic!("no input");
    }

    if dbg {
        println!("ranges  = {:?}", pairs);
        for k in pairs.iter() {
            // RangeInclusive::<u64>.len() is not implemented due to
            // the exception/panic of 0..=MAX
            println!(" {:?} : {} elements", k, 1 + k.end() - k.start());
        }
    }
    let mut total_invalid = 0;
    for k in pairs.iter() {
        println!("testing {:?}", k);
        total_invalid += sum_invalid_ids(k.clone());
    }
    println!("== Part 1: {total_invalid}");

    let mut total_invalid2 = 0;
    for k in pairs.iter() {
        total_invalid2 += sum_invalid_ids_any(k.clone());
    }
    println!("== Part 2: {total_invalid2}");
}
