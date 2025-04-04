// src/lib.rs

extern crate clap;
extern crate itertools;

use clap::{Parser, Subcommand};

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

/// The main library entry point.  
/// Calling this will parse command-line arguments and execute the appropriate commands.
pub fn run() {
    let args = Args::parse();
    args.run();
}
