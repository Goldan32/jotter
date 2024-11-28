mod cli;
mod command;
mod utils;

fn main() {
    let c = cli::get_command();
    println!("{:?}", c);
}
