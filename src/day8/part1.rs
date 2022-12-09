use super::*;

pub fn count_visible_trees() -> TimedResult<usize> {
    let start_time = Instant::now();

    let mut map = Map::from_file("inputs/day8.txt")?;

    // Mark the top edge as visible.
    for x in 0..map.width {
        map.tree_mut(x, 0).visible = true;
    }

    // Check the intermediate rows.
    for y in 1..map.height - 1 {
        // The left edge is always visible.
        map.tree_mut(0, y).visible = true;
        // Going from left to right, find what's visible and what's not.
        let mut tallest_tree = map.tree(0, y).height;
        for x in 1..map.width - 1 {
            if map.tree(x, y).height > tallest_tree {
                map.tree_mut(x, y).visible = true;
                tallest_tree = map.tree(x, y).height;
            }
        }

        // Now go from right to left.
        map.tree_mut(map.width - 1, y).visible = true;
        tallest_tree = map.tree(map.width - 1, y).height;
        for x in (1..map.width - 1).rev() {
            if map.tree(x, y).height > tallest_tree {
                map.tree_mut(x, y).visible = true;
                tallest_tree = map.tree(x, y).height;
            }
        }
    }

    // Mark the bottom edge as visible.
    for x in 0..map.width {
        map.tree_mut(x, map.height - 1).visible = true;
    }

    // Mark the left edge as visible.
    for y in 0..map.height {
        map.tree_mut(0, y).visible = true;
    }

    // Check the intermediate columns.
    for x in 1..map.width - 1 {
        // From top to bottom...
        map.tree_mut(x, 0).visible = true;
        let mut tallest_tree = map.tree(x, 0).height;
        for y in 1..map.height - 1 {
            if map.tree(x, y).height > tallest_tree {
                map.tree_mut(x, y).visible = true;
                tallest_tree = map.tree(x, y).height;
            }
        }

        // ... and from bottom to top!
        map.tree_mut(x, map.height - 1).visible = true;
        tallest_tree = map.tree(x, map.height - 1).height;
        for y in (1..map.height - 1).rev() {
            if map.tree(x, y).height > tallest_tree {
                map.tree_mut(x, y).visible = true;
                tallest_tree = map.tree(x, y).height;
            }
        }
    }

    // Mark the right edge as visible.
    for y in 0..map.height {
        map.tree_mut(map.width - 1, y).visible = true;
    }

    // Count the visible trees.
    let mut visible_trees = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.tree(x, y).visible {
                visible_trees += 1;
            }
        }
    }

    Ok((visible_trees, start_time.elapsed()))
}
