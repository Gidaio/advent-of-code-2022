use super::*;

const TOTAL_SIZE: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;

pub fn find_directory_to_delete() -> TimedResult<usize> {
    let file = fs::File::open("inputs/day7.txt")?;
    let start_time = time::Instant::now();

    let root = TreeBuilder::build_from_file(file)?;
    let current_free_space = TOTAL_SIZE - root.borrow().size;
    let free_space_needed = UPDATE_SIZE - current_free_space;

    Ok((
        find_minimum_big_enough(&root, free_space_needed),
        start_time.elapsed(),
    ))
}

fn find_minimum_big_enough(node: &MagicNode, target_size: usize) -> usize {
    let mut minimum = usize::MAX;
    let inner_node = node.borrow();

    for child_node in &inner_node.contents {
        if child_node.borrow().is_directory {
            let potential_minimum = find_minimum_big_enough(child_node, target_size);
            minimum = if potential_minimum < minimum && potential_minimum >= target_size {
                potential_minimum
            } else {
                minimum
            };
        }
    }

    if inner_node.size < minimum && inner_node.size >= target_size {
        inner_node.size
    } else {
        minimum
    }
}
