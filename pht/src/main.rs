use std::process::exit;
use std::{fs, io};
use std::fs::File;
use std::io::{BufReader, BufRead};
use chrono;
use reqwest;
use std::time::Instant;
use clap::Parser;
use csv;

// this is fine if you download only one file,
// for multiple files better is to make reqwest::blocking::Client::
// my experience is 3x performance for 50 files
fn _download_file_to(url: &str, to: &str) {
    let resp = reqwest::blocking::get(url).unwrap();
    let body = resp.text().unwrap();
    let mut out = File::create(to).unwrap();
    io::copy(&mut body.as_bytes(), &mut out).unwrap();
}

struct Download {
    client: reqwest::blocking::Client,
}

impl Download {
    fn new() -> Download {
        Download{ client: reqwest::blocking::Client::builder().build().unwrap(), }
    }
    
    fn download_file_to(&self, url: &str, to: &str) {
        let resp = self.client.get(url).send().unwrap();
        let body = resp.text().unwrap();
        let mut out = File::create(to).unwrap();
        io::copy(&mut body.as_bytes(), &mut out).unwrap();
    }
}

#[derive(Parser, Default, Debug)]
#[clap(author="Sasa Buklijas", version, about="Download data.\nAll parameters are optional, but at least one must be present.", arg_required_else_help=true)]
struct Arguments {
    #[clap(short, long, default_value_t = false)]
    /// areas.csv to download, default No
    areas: bool,

    #[clap(short, long, default_value_t = false)]
    /// markers.csv to download, default No
    markers: bool,

    #[clap(short, long, default_value_t = false)]
    /// tracks.csv to download, default No
    tracks: bool,

    #[clap(short, long)]
    /// path to file of GPS tracks to download, default No
    gpx_list_file: Option<String>,

    #[clap(short='l', long)]
    // lines from tracks.csv(must be in same dir as pth_download) in 11-22 format
    gpx_tracks_lines: Option<String>,
}

fn main() {
    let args = Arguments::parse();
    
    /*
    if (args.areas, args.markers, args.tracks, args.gpx_list_file.is_none()) == (false, false, false, true) {
        println!("Nothing to download.");
        println!("{:?}", args);
        exit(5);
    }*/

    let downloader = Download::new();

    // make dir
    let now = chrono::offset::Local::now();
    let custom_datetime_format = now.format("%Y%m%y_%H%M%S");
    let _new_dir = fs::create_dir(format!("{}", custom_datetime_format)).unwrap(); 
    println!("Downloading data to {:}/* folder", format!("{}", custom_datetime_format));

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
            downloader.download_file_to(&url, &file_path);
        }
        let duration = start.elapsed();
        println!("Download of {:?} took: {:?}", to_download, duration);
    }

    // download gpx_list_file
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
            //provjeri za 404 gresku !!!!, jer je prvi iz excela
            downloader.download_file_to(&url, &file_path); 
        }
        let duration = start.elapsed();
        println!("Download of {} GPX files took: {:?}", gpx_files.len(), duration);
    }

    // download gpx_tracks_lines from tracks.csv
    if args.gpx_tracks_lines.is_some() {
        let gpx_tracks_lines = args.gpx_tracks_lines.unwrap();
        let gpx_tracks_lines = gpx_tracks_lines.split("-");
        let gpx_tracks_lines: Vec<_> = gpx_tracks_lines.collect();
        //println!("{:?}", gpx_tracks_lines);

        let start;
        let end;
        let len_gpx_tracks_lines = gpx_tracks_lines.len();
        if len_gpx_tracks_lines == 1 {
            // TODO: add all option to download all, start=1, end. u32.max. Not best but fine
            start = gpx_tracks_lines[0].parse::<i32>().unwrap();
            end = gpx_tracks_lines[0].parse::<i32>().unwrap();
        } else if len_gpx_tracks_lines == 2 {
            start = gpx_tracks_lines[0].parse::<i32>().unwrap();
            end = gpx_tracks_lines[1].parse::<i32>().unwrap();
        } else {
            println!("Error format for --gpx_tracks_lines {:?}, only 2 numbers allowed separated by '-', like 3-27", gpx_tracks_lines);
            exit(5);
        }
        //println!("{:?} {:?}", start, end);

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .from_path("./tracks.csv").unwrap();

        let start_t = Instant::now();
        let mut line_counter = 0;
        for record in reader.records() {
            line_counter += 1;
            //println!("line_counter={} start={} end={}", line_counter, start, end);

            if line_counter < start {
                continue;
            }
            else if line_counter > end {
                let duration = start_t.elapsed();
                println!("Download of {} GPX files took: {:?}", end-start+1, duration);
                break;
            }
            else {
                let record = record.unwrap();
                //println!("{}  ---  {:?}", line_counter, record);

                let trail_name = record[2].to_string().replace(" ", "_").replace(":", "_").replace("/", "_");
                let area_name = record[4].to_string();
                let gpx_filename = record[0].to_string();
                //println!("{} --- {} --- {}", trail_name, area_name, gpx_filename);
                //let file = format!("{}/{}___{}", custom_datetime_format, area_name, trail_name);
                
                // FIXME: just {}{} / is already there, but it is working fine with it also working fine, do not know why, or maybe just to leave it 
                let url: String = format!("{}/{}", "https://www.hps.hr/karta/gpx/", gpx_filename);
                let file_path = format!("{}/{}___{}.gpx", custom_datetime_format, area_name, trail_name);
                //println!("{} --- {}", url, file_path);
                downloader.download_file_to(&url, &file_path); 
            }

            //if line_counter > 5 { exit(5); } // quick DEBUG
        }
    } 
}
