use super::*;

pub fn find_directory_sizes() -> TimedResult<usize> {
    let file = fs::File::open("inputs/day7.txt")?;
    let start_time = time::Instant::now();

    let root = TreeBuilder::build_from_file(file)?;

    Ok((traverse_tree(&root, 0), start_time.elapsed()))
}

fn traverse_tree(node: &MagicNode, initial_sum: usize) -> usize {
    let mut sum = initial_sum;
    let inner_node = node.borrow();

    for child_node in &inner_node.contents {
        if child_node.borrow().is_directory {
            sum += traverse_tree(child_node, initial_sum);
        }
    }

    if inner_node.size <= 100_000 {
        sum + inner_node.size
    } else {
        sum
    }
}
