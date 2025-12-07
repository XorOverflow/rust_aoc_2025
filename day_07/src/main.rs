/*
https://adventofcode.com/2025/day/7
--- Day 7: Laboratories ---
 */

use aoc::args::{is_debug, is_verbose};
use aoc::grid::{Grid, GridBuilder};
use std::io;
use std::io::prelude::*;

fn split_beams(splitters: &Grid<bool>, start: usize) -> usize {
    let width = splitters.width;
    let height = splitters.height;
    let mut beam_row: Vec<bool> = vec![false; width];
    let mut beam_next: Vec<bool> = vec![false; width];

    beam_row[start] = true;
    let mut splits = 0;

    // beam_row is the current set of beams going downwards,
    // to the next row "Y", and their state (splitted or forwared)
    // which will be stored in beam_next.
    for y in 1..height {
        for x in 1..width - 1 {
            if beam_row[x] {
                if splitters.get(x, y) {
                    splits += 1;
                    beam_next[x - 1] = true;
                    beam_next[x + 1] = true;
                } else {
                    beam_next[x] = true;
                }
            }
        }
        beam_row = beam_next;
        beam_next = vec![false; width];
    }
    splits
}

fn timelines(splitters: &Grid<bool>, start: usize) -> usize {
    let dbg = is_verbose();

    let width = splitters.width;
    let height = splitters.height;
    let mut timeline_row: Vec<usize> = vec![0; width];
    let mut timeline_next: Vec<usize> = vec![0; width];

    timeline_row[start] = 1;

    for y in 1..height {
        for x in 0..width {
            let t = timeline_row[x];
            if t > 0 {
                if splitters.get(x, y) {
                    // split: each of the previous
                    // timelines reaching here get doubled.
                    // They are added to the other timelines
                    // that got split from a different path.
                    timeline_next[x - 1] += t; // may need to add to a left split
                    timeline_next[x + 1] = t; // we are first, no need to add
                } else {
                    // no split: same timeline; but can be added
                    // to a different path.
                    timeline_next[x] += t;
                }
            }
        }
        timeline_row = timeline_next;
        timeline_next = vec![0; width];
        if dbg {
            eprintln!("{:?}", timeline_row);
        }
    }

    // Now add all final timelines together
    timeline_row.iter().sum()
}

fn main() {
    let dbg = aoc::args::is_debug();

    let mut gb = GridBuilder::<bool>::new();
    let mut start: usize = 0;

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        gb.append_char_map(&line, '^');
        // The first line must always contain an S.
        // Input is ASCII, so find(), which returns byte offset to utf8 char,
        // will be the correct X coordinate too.
        if start == 0 {
            start = line.find('S').unwrap();
        }
    }
    let splitters = gb.to_grid();
    if is_debug() {
        splitters.pretty_print_bool_micro();
    }

    let splits = split_beams(&splitters, start);
    println!("Part 1 = {splits}");

    let tl = timelines(&splitters, start);
    println!("Part 2 = {tl}");
}
