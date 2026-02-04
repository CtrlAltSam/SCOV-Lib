use scov_lib::run;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let paths = fs::read_dir(args[1].clone()).unwrap();
        
        run(paths, args[1].clone());
    } else {
        println!("No additional arguments provided.");
    }
}
