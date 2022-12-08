use super::*;

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

pub struct TreeBuilder {
    current_node: MagicNode,
}

impl TreeBuilder {
    pub fn build_from_file(file: fs::File) -> BoxedResult<MagicNode> {
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
