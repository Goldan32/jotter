mod frontend;
mod mw;
mod utils;

fn main() {
    let c = frontend::cli::get_command(std::env::args_os());
    println!("{:?}", c);
}
