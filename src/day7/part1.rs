use std::cell::RefCell;
use std::error;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};
use std::rc::Rc;
use std::time;

use super::*;

type MagicNode = Rc<RefCell<Node>>;

#[derive(Debug)]
enum TreeBuilderError {
    NoSuchNode,
    UnrecognizedCommand,
    MissingCommand,
    MissingName,
}

impl fmt::Display for TreeBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::NoSuchNode => write!(f, "No such node."),
            Self::UnrecognizedCommand => write!(f, "Unrecognized command."),
            Self::MissingCommand => write!(f, "Missing command."),
            Self::MissingName => write!(f, "Missing name."),
        }
    }
}

impl error::Error for TreeBuilderError {}

struct TreeBuilder {
    current_node: MagicNode,
}

impl TreeBuilder {
    fn build_from_file(file: fs::File) -> BoxedResult<MagicNode> {
        let reader = io::BufReader::new(file);

        let root_node = Node::create_directory("/");

        let mut terminal = Self {
            current_node: Rc::clone(&root_node),
        };

        for line in reader.lines() {
            let line = line?;
            let mut tokens = line.split_whitespace();

            while let Some(token) = tokens.next() {
                if token == "$" {
                    let command = tokens.next().ok_or(TreeBuilderError::MissingCommand)?;
                    if command == "cd" {
                        terminal
                            .change_node(tokens.next().ok_or(TreeBuilderError::MissingName)?)?;
                    } else if command != "ls" {
                        return Err(TreeBuilderError::UnrecognizedCommand.into());
                    }
                } else if token == "dir" {
                    terminal.add_node(Node::create_directory(
                        tokens.next().ok_or(TreeBuilderError::MissingName)?,
                    ));
                } else {
                    let file_size = token.parse::<usize>()?;
                    let file_name = tokens.next().ok_or(TreeBuilderError::MissingName)?;
                    terminal.add_node(Node::create_file(file_name, file_size));
                }
            }
        }

        Ok(root_node)
    }

    fn change_node(&mut self, target: &str) -> Result<(), TreeBuilderError> {
        if target == "/" {
            // This only happens at the beginning, so we can just ignore it.
            return Ok(());
        }

        let current_node = Rc::clone(&self.current_node);
        let current_node = current_node.borrow();

        if target == ".." {
            if let Some(parent) = &current_node.parent {
                self.current_node = Rc::clone(parent);
                return Ok(());
            } else {
                return Err(TreeBuilderError::NoSuchNode);
            }
        }

        for node in &current_node.contents {
            let inner_node = node.borrow();
            if inner_node.is_directory && inner_node.name == target {
                self.current_node = Rc::clone(node);
                return Ok(());
            }
        }

        Err(TreeBuilderError::NoSuchNode)
    }

    fn add_node(&self, node: MagicNode) {
        let mut inner_node = node.borrow_mut();
        let mut current_node = self.current_node.borrow_mut();

        current_node.contents.push(Rc::clone(&node));
        current_node.add_size(inner_node.size);
        inner_node.parent = Some(Rc::clone(&self.current_node));
    }
}

struct Node {
    is_directory: bool,
    name: String,
    size: usize,
    contents: Vec<MagicNode>,
    parent: Option<MagicNode>,
}

impl Node {
    fn create_file(name: &str, size: usize) -> MagicNode {
        Rc::new(RefCell::new(Self {
            is_directory: false,
            name: String::from(name),
            size,
            contents: vec![],
            parent: None,
        }))
    }

    fn create_directory(name: &str) -> MagicNode {
        Rc::new(RefCell::new(Self {
            is_directory: true,
            name: String::from(name),
            size: 0,
            contents: vec![],
            parent: None,
        }))
    }

    fn add_size(&mut self, size: usize) {
        self.size += size;

        if let Some(parent) = &self.parent {
            parent.borrow_mut().add_size(size);
        }
    }
}

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
