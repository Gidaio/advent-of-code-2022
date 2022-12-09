use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::TimedResult;

#[derive(Debug)]
struct Tree {
    height: u8,
    visible: bool,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.height, if self.visible { 'O' } else { '-' })
    }
}

const ASCII_ZERO: u8 = '0' as u8;

pub fn count_visible_trees() -> TimedResult<usize> {
    let start_time = Instant::now();

    let mut map: Vec<Vec<Tree>> = Vec::new();

    let file = File::open("inputs/day8.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        let trees = line?
            .into_bytes()
            .into_iter()
            .map(|item| Tree {
                height: item - ASCII_ZERO,
                visible: false,
            })
            .collect();
        map.push(trees);
    }

    let map_height = map.len();
    let map_width = map[0].len();

    // Mark the top edge as visible.
    for x in 0..map_width {
        map[0][x].visible = true;
    }

    // Check the intermediate rows.
    for y in 1..map_height - 1 {
        // The left edge is always visible.
        map[y][0].visible = true;
        // Going from left to right, find what's visible and what's not.
        let mut tallest_tree = map[y][0].height;
        for x in 1..map_width - 1 {
            if map[y][x].height > tallest_tree {
                map[y][x].visible = true;
                tallest_tree = map[y][x].height;
            }
        }

        // Now go from right to left.
        map[y][map_width - 1].visible = true;
        tallest_tree = map[y][map_width - 1].height;
        for x in (1..map_width - 1).rev() {
            if map[y][x].height > tallest_tree {
                map[y][x].visible = true;
                tallest_tree = map[y][x].height;
            }
        }
    }

    // Mark the bottom edge as visible.
    for x in 0..map_width {
        map[map_height - 1][x].visible = true;
    }

    // Mark the left edge as visible.
    for y in 0..map_height {
        map[y][0].visible = true;
    }

    // Check the intermediate columns.
    for x in 1..map_width - 1 {
        // The top edge is always visible.
        map[0][x].visible = true;
        // Going from top to bottom, find what's visible and what's not.
        let mut tallest_tree = map[0][x].height;
        for y in 1..map_height - 1 {
            if map[y][x].height > tallest_tree {
                map[y][x].visible = true;
                tallest_tree = map[y][x].height;
            }
        }

        // Now go from bottom to top.
        map[map_height - 1][x].visible = true;
        tallest_tree = map[map_height - 1][x].height;
        for y in (1..map_height - 1).rev() {
            if map[y][x].height > tallest_tree {
                map[y][x].visible = true;
                tallest_tree = map[y][x].height;
            }
        }
    }

    // Mark the left edge as visible.
    for y in 0..map_height {
        map[y][map_width - 1].visible = true;
    }

    // Count the visible trees.
    let mut visible_trees = 0;
    for y in 0..map_height {
        for x in 0..map_width {
            if map[y][x].visible {
                visible_trees += 1;
            }
        }
    }

    Ok((visible_trees, start_time.elapsed()))
}
