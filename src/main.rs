use std::env;

fn main() {
    let mut args = env::args();
    args.next(); // ignore 1st arg
    if let Some(command) = args.next() {
        run_command(command);
    } else {
        println!("no argument provided");
    }
}

fn run_command(command: String) {
    if command == "-d" {
        run_download_test();
    } else if command == "-u" {
        run_upload_test();
    } else {
        println!("invalid argument");
    }
}

fn run_download_test() {
    println!("run download test");
}

fn run_upload_test() {
    println!("run upload test");
}
