use flate2::read::GzDecoder;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::str;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use tar::Archive;

const SPEEDTEST_URL: &str = "https://bintray.com/ookla/download/download_file?file_path=ookla-speedtest-1.0.0-x86_64-linux.tgz";
const TARBALL_NAME: &str = "speedtest.tgz";
const SPEEDTEST_BIN_NAME: &str = "speedtest";

fn main() {
    test_speed();
}

fn test_speed() {
    let temp_dir_name = create_temp_dir_name().expect("error creating temp directory name");
    create_temp_dir(&temp_dir_name).expect("error creating temp directory");
    download_speedtest_cli(&temp_dir_name, TARBALL_NAME).expect("error downloading speedtest");
    extract_speedtest_cli(&temp_dir_name, TARBALL_NAME).expect("error extracting speedtest");
    let command = format!(
        "./{}/{} -f json --accept-license --accept-gdpr",
        &temp_dir_name, SPEEDTEST_BIN_NAME
    );
    let json_result = run_speedtest_cli(&command).expect("error running speedtest");
    println!("...done\n");
    print_speedtest_results(&json_result).expect("error reading results");
    clean_files(&temp_dir_name).expect(&format!(
        "Unable to clean files. Please remove directory: {}",
        temp_dir_name
    ));
}

fn create_temp_dir_name() -> Result<String, SystemTimeError> {
    Ok(format!(
        "temp_{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
    ))
}

fn create_temp_dir(dir_name: &str) -> std::io::Result<()> {
    fs::create_dir(Path::new(&dir_name))?;
    Ok(())
}

fn download_speedtest_cli(dir_name: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    println!("\ndownloading speedtest...");
    let file_full_path = Path::new(dir_name).join(file_name);
    let speedtest_bin = reqwest::blocking::get(SPEEDTEST_URL)?.bytes()?;
    fs::write(file_full_path, speedtest_bin)?;
    Ok(())
}

fn extract_speedtest_cli(dir_name: &str, file_name: &str) -> Result<(), std::io::Error> {
    println!("extracting speedtest...");
    let file_full_path = Path::new(dir_name).join(file_name);
    let tar_gz = fs::File::open(file_full_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(Path::new(dir_name))?;
    Ok(())
}

fn clean_files(dir_name: &str) -> Result<(), std::io::Error> {
    let dir_path = Path::new(dir_name);
    fs::remove_dir_all(dir_path)?;
    Ok(())
}

fn run_speedtest_cli(command_name: &str) -> Result<String, Box<dyn Error>> {
    println!("running speedtest...");
    let stdout = Command::new("sh")
        .arg("-c")
        .arg(command_name)
        .output()?
        .stdout;
    Ok(String::from_utf8(stdout)?)
}

fn print_speedtest_results(json_results: &str) -> Result<(), Box<dyn Error>> {
    let v: Value = serde_json::from_str(json_results)?;
    let download = v["download"]["bandwidth"].as_f64().unwrap();
    let upload = v["upload"]["bandwidth"].as_f64().unwrap();
    let latency = v["ping"]["latency"].as_f64().unwrap();
    let isp = v["isp"].as_str().unwrap();
    let server = &v["server"];
    let server_ip = server["ip"].as_str().unwrap();
    let server_country = server["country"].as_str().unwrap();
    let server_location = server["location"].as_str().unwrap();
    println!("isp: {}", isp);
    println!(
        "download: {} Mbps",
        convert_bits_to_readable_megabits(download)
    );
    println!("upload: {} Mbps", convert_bits_to_readable_megabits(upload));
    println!("latency: {} ms", format!("{:.2}", latency));
    println!(
        "test server: {}, {} {}",
        server_location, server_country, server_ip
    );
    Ok(())
}

fn convert_bits_to_readable_megabits(bits: f64) -> String {
    let megabits = bits / 125000_f64;
    format!("{:.2}", megabits)
}
