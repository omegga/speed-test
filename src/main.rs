use flate2::read::GzDecoder;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::process::Command;
use std::str;
use tar::Archive;

const SPEEDTEST_URL: &str = "https://bintray.com/ookla/download/download_file?file_path=ookla-speedtest-1.0.0-x86_64-linux.tgz";
const TARBALL: &str = "speedtest.tgz";
const SPEEDTEST_BIN: &str = "speedtest";

fn main() {
    test_speed();
}

fn test_speed() {
    download_speedtest_cli(TARBALL).expect("error downloading speedtest");
    extract_speedtest_cli(TARBALL).expect("error extracting speedtest");
    let command = format!("./{} -f json", SPEEDTEST_BIN);
    let json_result = run_speedtest_cli(&command).expect("error running speedtest");
    println!("...done\n");
    print_speedtest_results(&json_result).expect("error reading results");
}

fn download_speedtest_cli(filename: &str) -> Result<(), Box<dyn Error>> {
    println!("\ndownloading speedtest...");
    let speedtest = reqwest::blocking::get(SPEEDTEST_URL)?.bytes()?;
    fs::write(filename, speedtest)?;
    Ok(())
}

fn extract_speedtest_cli(filename: &str) -> Result<(), std::io::Error> {
    println!("extracting speedtest...");
    let tar_gz = fs::File::open(filename)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
}

fn run_speedtest_cli(command: &str) -> Result<String, Box<dyn Error>> {
    println!("running speedtest...");
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute speedtest");
    Ok(String::from_utf8(output.stdout).expect("failed to convert output"))
}

fn print_speedtest_results(json_results: &str) -> Result<(), Box<dyn Error>> {
    let v: Value = serde_json::from_str(json_results)?;
    let download = v["download"]["bandwidth"].as_f64().unwrap();
    let upload = v["upload"]["bandwidth"].as_f64().unwrap();
    let latency = v["ping"]["latency"].as_f64().unwrap();
    let isp = v["isp"].as_str().unwrap();
    let server_ip = v["server"]["ip"].as_str().unwrap();
    let server_country = v["server"]["country"].as_str().unwrap();
    let server_location = v["server"]["location"].as_str().unwrap();
    println!("isp: {}", isp);
    println!("download: {} Mbps", convert_bits_to_megabits(download));
    println!("upload: {} Mbps", convert_bits_to_megabits(upload));
    println!("latency: {} ms", format!("{:.2}", latency));
    println!(
        "server: {}",
        format!("{}, {} {}", server_location, server_country, server_ip)
    );
    Ok(())
}

fn convert_bits_to_megabits(bits: f64) -> String {
    let megabits = bits / (125000 as f64);
    format!("{:.2}", megabits)
}
