pub mod part1;
pub mod part2;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const START: u8 = 'S' as u8;
const END: u8 = 'E' as u8;
const LOWEST: u8 = 'a' as u8 - 1;

#[derive(Debug)]
struct Dijkstra {
    position: (usize, usize),
    distance: usize,
}

impl PartialEq for Dijkstra {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Dijkstra {}

impl PartialOrd for Dijkstra {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.distance < other.distance {
            Some(Ordering::Greater)
        } else if self.distance > other.distance {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Dijkstra {
    fn cmp(&self, other: &Self) -> Ordering {
        // Switch these because we want a min heap.
        if self.distance < other.distance {
            Ordering::Greater
        } else if self.distance > other.distance {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Debug)]
struct Map {
    heightmap: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl TryFrom<File> for Map {
    type Error = Box<dyn Error>;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let reader = BufReader::new(value);

        let mut heightmap = Vec::<Vec<u8>>::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut x = 0;
        let mut y = 0;

        for line in reader.lines() {
            let mut row = Vec::<u8>::new();
            for byte in line?.bytes() {
                row.push(match byte {
                    START => {
                        start = (x, y);
                        1
                    }
                    END => {
                        end = (x, y);
                        26
                    }
                    _ => byte - LOWEST,
                });

                x += 1;
            }

            heightmap.push(row);
            x = 0;
            y += 1;
        }

        Ok(Self {
            heightmap,
            start,
            end,
        })
    }
}

impl Map {
    fn find_shortest_path(&self) -> usize {
        let mut shortest = usize::MAX;

        for y in 0..self.heightmap.len() {
            for x in 0..self.heightmap[y].len() {
                if self.heightmap[y][x] == 1 {
                    if let Some(length) = self.find_shortest_path_from((x, y)) {
                        if length < shortest {
                            shortest = length;
                        }
                    }
                }
            }
        }

        shortest
    }

    fn find_shortest_path_from(&self, start: (usize, usize)) -> Option<usize> {
        let mut visited = HashSet::<(usize, usize)>::new();
        let mut to_visit = BinaryHeap::<Dijkstra>::new();
        let start = Dijkstra {
            position: start,
            distance: 0,
        };

        to_visit.push(start);

        while let Some(current) = to_visit.pop() {
            if current.position == self.end {
                return Some(current.distance);
            }

            let current_height = self.heightmap[current.position.1][current.position.0];

            if current.position.1 > 0 {
                let up_position = (current.position.0, current.position.1 - 1);
                if !visited.contains(&up_position) {
                    let up_height = self.heightmap[up_position.1][up_position.0];
                    if up_height <= current_height + 1 {
                        to_visit.push(Dijkstra {
                            position: up_position,
                            distance: current.distance + 1,
                        });
                        visited.insert(up_position);
                    }
                }
            }

            if current.position.0 > 0 {
                let left_position = (current.position.0 - 1, current.position.1);
                if !visited.contains(&left_position) {
                    let left_height = self.heightmap[left_position.1][left_position.0];
                    if left_height <= current_height + 1 {
                        to_visit.push(Dijkstra {
                            position: left_position,
                            distance: current.distance + 1,
                        });
                        visited.insert(left_position);
                    }
                }
            }

            if current.position.1 < self.heightmap.len() - 1 {
                let down_position = (current.position.0, current.position.1 + 1);
                if !visited.contains(&down_position) {
                    let down_height = self.heightmap[down_position.1][down_position.0];
                    if down_height <= current_height + 1 {
                        to_visit.push(Dijkstra {
                            position: down_position,
                            distance: current.distance + 1,
                        });
                        visited.insert(down_position);
                    }
                }
            }

            if current.position.0 < self.heightmap[0].len() - 1 {
                let right_position = (current.position.0 + 1, current.position.1);
                if !visited.contains(&right_position) {
                    let right_height = self.heightmap[right_position.1][right_position.0];
                    if right_height <= current_height + 1 {
                        to_visit.push(Dijkstra {
                            position: right_position,
                            distance: current.distance + 1,
                        });
                        visited.insert(right_position);
                    }
                }
            }
        }

        None
    }
}
