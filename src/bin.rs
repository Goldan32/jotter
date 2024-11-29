mod cli;
mod command;
mod task;
mod utils;

fn main() {
    let c = cli::get_command(std::env::args_os());
    println!("{:?}", c);
}
