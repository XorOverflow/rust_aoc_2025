/*
https://adventofcode.com/2025/day/4
--- Day 4: Printing Department ---
 */

use aoc::grid::{Grid, GridBuilder};
use std::io;
use std::io::prelude::*;
use std::time::{Duration, Instant};

fn count_accessible_rolls_and_remove(map: &Grid<bool>) -> (usize, Grid<bool>) {
    let mut newmap = map.clone();

    let w = map.width as isize;
    let h = map.height as isize;
    let mut count: usize = 0;
    for y in 0..h {
        for x in 0..w {
            // count only rolls
            if !map.get(x as usize, y as usize) {
                continue;
            }
            let mut neighbors = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        // We could also not make this test
                        // and just adjust the total neighbors limit
                        // with ourself, so total 5
                        continue;
                    }
                    if map.checked_get_or_default(x + dx, y + dy, false) {
                        neighbors += 1;
                    }
                }
            }
            if neighbors < 4 {
                // roll accessible
                count += 1;
                // remove it for the next step
                newmap.set(x as usize, y as usize, false);
            }
        }
    }

    (count, newmap)
}

fn main() {
    let dbg = aoc::args::is_debug();

    let start_parse = Instant::now(); // Start measuring time.
    let mut gb = GridBuilder::<bool>::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        gb.append_char_map(&line, '@');
    }
    let map = gb.to_grid();
    if dbg {
        map.pretty_print_bool_micro();
    }

    let elapsed_parse: Duration = Instant::now() - start_parse; // Calculate elapsed time.

    let start_process = Instant::now(); // Start measuring time.

    let (mut accessible, mut next_map) = count_accessible_rolls_and_remove(&map);

    println!("Part 1 = {accessible}");

    let mut total_accessible = accessible;
    while accessible != 0 {
        (accessible, next_map) = count_accessible_rolls_and_remove(&next_map);
        total_accessible += accessible;
    }

    println!("Part 2 = {total_accessible}");

    let elapsed_process: Duration = Instant::now() - start_process; // Calculate elapsed time.
    eprintln!("Time taken for parsing: {:?}", elapsed_parse);
    eprintln!("Time taken for processing: {:?}", elapsed_process);
    eprintln!("Total time: {:?}", elapsed_process + elapsed_parse);
}
