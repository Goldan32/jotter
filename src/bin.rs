mod cli;
mod command;
mod utils;

fn main() {
    let c = cli::get_command(std::env::args_os());
    println!("{:?}", c);
}
