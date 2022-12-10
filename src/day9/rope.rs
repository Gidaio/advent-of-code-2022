use std::collections::HashSet;

use super::*;

pub struct Rope {
    position: Vector2,
    past_positions: HashSet<Vector2>,
    next: Option<Box<Rope>>,
}

impl Rope {
    pub fn new(len: usize) -> Self {
        if len == 0 {
            Self {
                position: Vector2::zero(),
                past_positions: HashSet::from([Vector2::zero()]),
                next: None,
            }
        } else {
            let rest = Rope::new(len - 1);
            Self {
                position: Vector2::zero(),
                past_positions: HashSet::from([Vector2::zero()]),
                next: Some(Box::new(rest)),
            }
        }
    }

    pub fn move_times_in_direction(&mut self, direction: Vector2, times: usize) {
        for _ in 0..times {
            self.move_in_direction(direction);
        }
    }

    fn move_in_direction(&mut self, direction: Vector2) {
        self.position += direction;
        self.past_positions.insert(self.position);

        if let Some(next) = &mut self.next {
            let diff = self.position - next.position;

            let move_direction = if diff.magnitude() >= 2 {
                Vector2::new(diff.x.signum(), diff.y.signum())
            } else {
                Vector2::zero()
            };

            if move_direction.x != 0 || move_direction.y != 0 {
                next.move_in_direction(move_direction);
            }
        }
    }

    pub fn get_tail_positions(&self) -> &HashSet<Vector2> {
        if let Some(next) = &self.next {
            next.get_tail_positions()
        } else {
            &self.past_positions
        }
    }
}
