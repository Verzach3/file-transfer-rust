use std::env;
use file_transfer_rust::{client, server};
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args[1] == "s" {
        server(&args[2], &args[3]).unwrap();
    }

    if args[1] == "c" {
        client(&args[2], &args[3]).unwrap();
    }

}
