use std::fs;
use std::io;
use std::fs::File;
use chrono;
use reqwest;

fn download_file_to(url: &str, to: &str) {
    let resp = reqwest::blocking::get(url).unwrap();
    let body = resp.text().unwrap();
    let mut out = File::create(to).unwrap();
    io::copy(&mut body.as_bytes(), &mut out).unwrap();
}

fn main() {
    let now = chrono::offset::Local::now();
    let custom_datetime_format = now.format("%Y%m%y_%H%M%S");
    println!("{:}", custom_datetime_format);

    let new_dir = fs::create_dir(format!("{}", custom_datetime_format)).unwrap(); 
    println!("New directory created");
    println!("{:?}", new_dir);

    // working, but currently do not need it
    //let to_download = vec!["areas.csv", "markers.csv", "tracks.csv",];

    for item in to_download {
        let url = format!("{}/{}", "https://www.hps.hr/karta/csv/", item);  
        let file_path = format!("{}/{}", custom_datetime_format, item);  
        download_file_to(&url, &file_path)
    }
    
    // generate list of gpx tracks
        // download one by one


}
