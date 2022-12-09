use super::*;

pub fn get_best_scenic_score() -> TimedResult<usize> {
    let start_time = Instant::now();

    let map = Map::from_file("inputs/day8.txt")?;

    let mut best_scenic_score: usize = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            let scenic_score = calculate_scenic_score_for_tree(&map, x, y);
            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }

    Ok((best_scenic_score, start_time.elapsed()))
}

fn calculate_scenic_score_for_tree(map: &Map, tree_x: usize, tree_y: usize) -> usize {
    let tree = map.tree(tree_x, tree_y);

    // Go right.
    let mut right_view: usize = 0;
    for x in tree_x + 1..map.width {
        right_view += 1;
        if map.tree(x, tree_y).height >= tree.height {
            break;
        }
    }

    // Go left.
    let mut left_view: usize = 0;
    for x in (0..tree_x).rev() {
        left_view += 1;
        if map.tree(x, tree_y).height >= tree.height {
            break;
        }
    }

    // Go down.
    let mut down_view: usize = 0;
    for y in tree_y + 1..map.height {
        down_view += 1;
        if map.tree(tree_x, y).height >= tree.height {
            break;
        }
    }

    // Go up.
    let mut up_view: usize = 0;
    for y in (0..tree_y).rev() {
        up_view += 1;
        if map.tree(tree_x, y).height >= tree.height {
            break;
        }
    }

    right_view * left_view * down_view * up_view
}
