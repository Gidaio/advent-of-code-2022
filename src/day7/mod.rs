mod node;
pub mod part1;
pub mod part2;
mod tree_builder;

use std::cell::RefCell;
use std::error;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};
use std::rc::Rc;
use std::time;

use crate::{BoxedResult, TimedResult};
use node::Node;
use tree_builder::TreeBuilder;

type MagicNode = Rc<RefCell<Node>>;
