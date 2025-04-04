// src/lib.rs

extern crate clap;
extern crate itertools;

use clap::{Parser, Subcommand};
use std::collections::{BTreeMap, HashSet};
use tree::tree_node::TreeNode;

pub mod test_utils;
pub mod tree;
pub mod parser;

const LONG_ABOUT: &str = r#"
A command line tool for drawing tree structures with ascii characters.

Example:

astree horizontal -i "$(cat << 'EOF'
# Root
## Child 1
### Grandchild 1
### Grandchild 2
EOF
)"
"#;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = LONG_ABOUT)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

impl Args {
    pub fn run(self) {
        match self.command {
            Command::Vertical(vertical_args) => vertical_args.run(),
            Command::Horizontal(horizontal_args) => horizontal_args.run(),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Print the tree vertically. Use 'v' for shorthand.
    #[clap(alias = "v")]
    Vertical(VerticalArgs),
    /// Print the tree horizontally. Use 'h' for shorthand.
    #[clap(alias = "h")]
    Horizontal(HorizontalArgs),
}

#[derive(Parser, Debug)]
pub struct HorizontalArgs {
    /// The input filename or content
    #[clap(short, long)]
    pub input: String,
}

impl HorizontalArgs {
    pub fn run(&self) {
        // Don't support automatically adding line breaks for horizontal tree.
        let root_nodes = parser::parse(&self.input, None);
        tree::horizontal::print_nodes_std(&root_nodes)
    }
}

#[derive(Parser, Debug)]
pub struct VerticalArgs {
    #[clap(short, long, value_enum, default_value = "thin")]
    pub style: tree::style::Style,

    /// The input filename or content
    #[clap(short, long)]
    pub input: String,

    /// The maximum width of each box
    #[clap(short, long)]
    pub width: Option<usize>,

    /// The horizontal spacing between boxes
    #[clap(long, default_value_t = 2)]
    pub spacing: usize,
}

impl VerticalArgs {
    pub fn run(self) {
        let root_nodes = parser::parse(&self.input, self.width);
        for root in root_nodes {
            let result = tree::vertical::render(
                &root,
                &tree::style::BoxDrawings::new(self.style),
                self.spacing,
            );
            println!("{}", result);
        }
    }
}

// Function to build the hierarchy string from a BTreeMap
fn btreemap_to_string(
  hierarchy: &BTreeMap<String, Vec<String>>, // Hierarchy data
  current: &String,                        // Current node
  level: usize,                         // Current level in the tree
  visited: &mut HashSet<String>,         // To avoid visiting the same node twice
  output: &mut String,                   // Output string to accumulate results
) {
  // If the current node has already been visited, return early
  if !visited.insert(current.to_string()) {
      return;
  }

  // Add the current node to the output string with the appropriate number of '#' characters
  output.push_str(&format!("{} {}\n", "#".repeat(level), current));

  // If there are child nodes for the current node
  if let Some(children) = hierarchy.get(current) {
      for child in children {
          // If the child has its own children, recursively call the function
          if hierarchy.contains_key(child) {
              btreemap_to_string(hierarchy, child, level + 1, visited, output);
          } else {
              // Otherwise, just add the child to the output with an additional level
              output.push_str(&format!("{} {}\n", "#".repeat(level + 1), child));
          }
      }
  }
}

// Public function to print the hierarchy as a string
pub fn btreemap_to_node(hierarchy: &BTreeMap<String, Vec<String>>, root: &String) -> Vec<TreeNode> {
  let mut output = String::new();
  let mut visited = HashSet::new();
  btreemap_to_string(hierarchy, root, 1, &mut visited, &mut output);

  parser::parse(&output, None)
}

/// The main library entry point.  
/// Calling this will parse command-line arguments and execute the appropriate commands.
pub fn run() {
    let args = Args::parse();
    args.run();
}
