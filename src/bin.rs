mod frontend;
mod mw;
mod utils;

fn main() {
    let executor = crate::mw::Middleware::new();
    executor.main();
}
