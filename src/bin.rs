mod frontend;
mod mw;
mod utils;

use crate::mw::ui::FrontEndInput;

fn main() {
    println!("{:?}", frontend::cli::Cli::execute());
}
