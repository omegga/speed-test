use std::env;
use std::fs;

const SPEEDTEST_URL: &str = "https://bintray.com/ookla/download/download_file?file_path=ookla-speedtest-1.0.0-x86_64-linux.tgz";

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
        let dest = "speedtest.tgz";
        download_speedtest(dest).expect("error downloading speedtest");
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

fn download_speedtest(dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let speedtest = reqwest::blocking::get(SPEEDTEST_URL)?.bytes()?;
    fs::write(dest, speedtest)?;
    println!("Done");
    Ok(())
}
