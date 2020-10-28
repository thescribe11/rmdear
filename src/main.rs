use std::env;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    match rmdear::delete(args) {
        // I already handle any possible errors in delete().
        Ok(_) => {}
        Err(_) => {}
    }
}