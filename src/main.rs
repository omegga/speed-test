use flate2::read::GzDecoder;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::process::Command;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use tar::Archive;

const SPEEDTEST_URL: &str = "https://bintray.com/ookla/download/download_file?file_path=ookla-speedtest-1.0.0-x86_64-linux.tgz";
const TARBALL: &str = "speedtest.tgz";
const SPEEDTEST_BIN: &str = "speedtest";

fn main() {
    test_speed();
}

fn test_speed() {
    let temp_dir = create_temp_dir().expect("failed to create temp directory");
    download_speedtest_cli(&temp_dir, TARBALL).expect("error downloading speedtest");
    extract_speedtest_cli(&temp_dir, TARBALL).expect("error extracting speedtest");
    let command = format!("./{}/{} -f json", &temp_dir, SPEEDTEST_BIN);
    let json_result = run_speedtest_cli(&command).expect("error running speedtest");
    println!("...done\n");
    print_speedtest_results(&json_result).expect("error reading results");
    clean_files(&temp_dir).expect("error cleaning files");
}

fn create_temp_dir() -> Result<String, std::io::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let dir_name = format!("temp_{}", now);
    fs::create_dir(std::path::Path::new(&dir_name))?;
    Ok(dir_name)
}

fn download_speedtest_cli(dir: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    println!("\ndownloading speedtest...");
    let full_path = std::path::Path::new(dir).join(filename);
    let speedtest = reqwest::blocking::get(SPEEDTEST_URL)?.bytes()?;
    fs::write(full_path, speedtest)?;
    Ok(())
}

fn extract_speedtest_cli(dir: &str, filename: &str) -> Result<(), std::io::Error> {
    println!("extracting speedtest...");
    let full_path = std::path::Path::new(dir).join(filename);
    let tar_gz = fs::File::open(full_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let dir_path = std::path::Path::new(dir);
    archive.unpack(dir_path)?;
    Ok(())
}

fn clean_files(dir: &str) -> Result<(), std::io::Error> {
    let dir_path = std::path::Path::new(dir);
    fs::remove_dir_all(dir_path)?;
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
    let megabits = bits / 125000_f64;
    format!("{:.2}", megabits)
}
