pub mod part1;
pub mod part2;

use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::{BoxedResult, TimedResult};

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

struct Map {
    width: usize,
    height: usize,
    trees: Vec<Vec<Tree>>,
}

impl Map {
    fn from_file(path: &str) -> BoxedResult<Self> {
        let mut grid: Vec<Vec<Tree>> = Vec::new();

        let file = File::open(path)?;
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
            grid.push(trees);
        }

        let height = grid.len();
        let width = grid[0].len();

        Ok(Self {
            width,
            height,
            trees: grid,
        })
    }

    fn tree(&self, x: usize, y: usize) -> &Tree {
        &self.trees[y][x]
    }

    fn tree_mut(&mut self, x: usize, y: usize) -> &mut Tree {
        &mut self.trees[y][x]
    }
}
