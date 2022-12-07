use std::process::exit;
use std::{fs, io};
use std::fs::File;
use std::io::{BufReader, BufRead};
use chrono;
use reqwest;
use std::time::Instant;
use clap::Parser;

fn download_file_to(url: &str, to: &str) {
    let resp = reqwest::blocking::get(url).unwrap();
    let body = resp.text().unwrap();
    let mut out = File::create(to).unwrap();
    io::copy(&mut body.as_bytes(), &mut out).unwrap();
}

#[derive(Parser, Default, Debug)]
#[clap(author="Sasa Buklijas", version, about="Download data")]
struct Arguments {
    #[clap(short, long, default_value_t = false)]
    /// if areas.csv to download
    areas: bool,
    #[clap(short, long, default_value_t = false)]
    /// if markers.csv to download
    markers: bool,
    #[clap(short, long, default_value_t = false)]
    /// if tracks.csv to download
    tracks: bool,
    #[clap(short, long)]
    /// path to file of GPS tracks to download
    gpx_list_file: Option<String>,
}

fn main() {
    let args = Arguments::parse();
    
    if (args.areas, args.markers, args.tracks, args.gpx_list_file.is_none()) == (false, false, false, true) {
        println!("Nothing to download.");
        println!("{:?}", args);
        exit(5);
    }

    // make dir
    let now = chrono::offset::Local::now();
    let custom_datetime_format = now.format("%Y%m%y_%H%M%S");
    let _new_dir = fs::create_dir(format!("{}", custom_datetime_format)).unwrap(); 

    // parse
    let mut to_download = Vec::new();
    if args.areas {
        to_download.push("areas.csv");
    }
    if args.markers {
        to_download.push("markers.csv");
    }
    if args.tracks {
        to_download.push("tracks.csv");
    }

    // download csv
    if to_download.len() != 0 {
        let start = Instant::now();
        for item in &to_download {
            let url = format!("{}/{}", "https://www.hps.hr/karta/csv/", item);  
            let file_path = format!("{}/{}", custom_datetime_format, item);  
            download_file_to(&url, &file_path);
        }
        let duration = start.elapsed();
        println!("Download of {:?} took: {:?}", to_download, duration);
    }

    if args.gpx_list_file.is_some() {
        // get GPX from file to vec
        let mut gpx_files = Vec::new(); 
        let input = File::open(args.gpx_list_file.unwrap()).unwrap();
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
}
