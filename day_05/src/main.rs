/*
https://adventofcode.com/2025/day/5
--- Day 5: Cafeteria ---
 */
use aoc::args::{is_debug, is_verbose};
use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn count_fresh_ingredients(ingredients: &Vec<u64>, fresh: &Vec<RangeInclusive<u64>>) -> usize {
    let mut count: usize = 0;
    // far more readable than any ingredients.iter().filter(|| fresh... ?).count()

    for i in ingredients.into_iter() {
        for f in fresh.into_iter() {
            if f.contains(i) {
                count += 1;
                break;
            }
        }
    }
    count
}

fn count_total_ranges(fresh: &Vec<RangeInclusive<u64>>) -> u64 {
    // Do an initial sort to simplify intersecting tests and fusing:
    // first elements will never be inside later elements.
    let mut sorted_fresh = fresh.clone();
    sorted_fresh.sort_by(|r1, r2| {
        if r1.start() < r2.start() {
            Ordering::Less
        } else if r1.start() > r2.start() {
            Ordering::Greater
        } else {
            let e = r2.end();
            r1.end().cmp(&e)
        }
    });

    let dbg = is_debug();
    if dbg {
        eprintln!("Sorted ranges = {:?}", sorted_fresh);
    }

    // Store a canonical representation of the ranges
    // without any overlap.
    let mut uniq_fresh = Vec::<RangeInclusive<u64>>::new();

    let mut total_uniq_count: u64 = 0;

    'next_f: for f in sorted_fresh {
        let mut f = f; // can be iteratively clipped.
        'uniq_test: for u in &uniq_fresh {
            if f.start() < u.start() {
                // impossible due to initial sorting
                panic!("impossible sort");
            }
            if f.start() > u.end() {
                // disjoint ranges, still valid.
                continue 'uniq_test;
            }
            if f.start() >= u.start() && f.end() <= u.end() {
                // f is completely inside a previous range:
                // skip it.
                continue 'next_f;
            }
            if f.start() <= u.end() {
                // f partially intersects. Remove common part.
                let new_start = u.end() + 1;
                if new_start <= *f.end() {
                    // update range
                    f = new_start..=*f.end();
                    continue;
                } else {
                    // range now empty, should have been caught by previous test
                    panic!("empty range checked too late");
                }
            }
        }
        // Store the new 'f' after possible clipping of common parts
        // from previous elements. We can already count its
        // range value.
        total_uniq_count += f.end() - f.start() + 1;
        if dbg {
            eprintln!("pushing uniq range {:?}", f);
        }
        uniq_fresh.push(f);
    }

    total_uniq_count
}

fn main() {
    let vrb = is_verbose();
    let mut fresh = Vec::<RangeInclusive<u64>>::new();
    let mut ingredients = Vec::<u64>::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        if line.len() == 0 {
            break;
        }
        let (r1, r2) = line.split_once('-').unwrap();
        let range = (u64::from_str(r1).unwrap())..=(u64::from_str(r2).unwrap());
        fresh.push(range);
    }

    while let Some(Ok(line)) = lines.next() {
        let i = u64::from_str(&line).unwrap();
        ingredients.push(i);
    }

    if vrb {
        eprintln!(" Fresh ranges = {:?}", fresh);
        eprintln!(" Ingredients = {:?}", ingredients);
    }

    let fresh_1 = count_fresh_ingredients(&ingredients, &fresh);
    println!("Part 1 = {fresh_1}");

    let total = count_total_ranges(&fresh);
    println!("Part 2 = {total}");
}
