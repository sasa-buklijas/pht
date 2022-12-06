use std::{fs, io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use chrono;
use reqwest;
use std::time::Instant;

fn download_file_to(url: &str, to: &str) {
    let resp = reqwest::blocking::get(url).unwrap();
    let body = resp.text().unwrap();
    let mut out = File::create(to).unwrap();
    io::copy(&mut body.as_bytes(), &mut out).unwrap();
}

fn main() {
    // make dir
    let now = chrono::offset::Local::now();
    let custom_datetime_format = now.format("%Y%m%y_%H%M%S");
    let _new_dir = fs::create_dir(format!("{}", custom_datetime_format)).unwrap(); 

    // download csv
    let to_download = vec!["areas.csv", "markers.csv", "tracks.csv",];
    let start = Instant::now();
    for item in &to_download {
        let url = format!("{}/{}", "https://www.hps.hr/karta/csv/", item);  
        let file_path = format!("{}/{}", custom_datetime_format, item);  
        download_file_to(&url, &file_path);
    }
    let duration = start.elapsed();
    println!("Download of {:?} took: {:?}", to_download, duration);

    // simple CLI parsing
    let args: Vec<String> = env::args().collect();
    let gpx_list_file = &args[1];

    // get GPX from file to vec
    let mut gpx_files = Vec::new(); 
    let input = File::open(gpx_list_file).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let gpx_file = line.unwrap();
        gpx_files.push(gpx_file);
    }

    // download GPX
    let start = Instant::now();
    for file in &gpx_files {
        let url = format!("{}/{}", "https://www.hps.hr/karta/gpx/", file);
        let file_path = format!("{}/{}", custom_datetime_format, file);
        download_file_to(&url, &file_path)
    }
    let duration = start.elapsed();
    println!("Download of {} GPX files took: {:?}", gpx_files.len(), duration);

}
