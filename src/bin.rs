mod cli;
mod time;

fn main() {
    bjl::greet();
    println!("Hello, world!");
    cli::cli_app();
}
