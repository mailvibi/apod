use reqwest;
use std::{fs::File, path::PathBuf};
use serde_json::{Value};
use std::{thread, time::Duration};
use process_path::get_executable_path;
use wallpaper;

fn main() {
    println!("cache dir {:#?}", dirs::cache_dir());
    const APOD_URL : &str = "https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY";
    const URL_KEY: &str = "hdurl";
    const WAIT_TIME_SEC : u64 = 60 * 60 * 5;

    loop {
        let res = reqwest::blocking::get(APOD_URL);
        if let Err(e) = res {
            println!("Received error : {}", e);
        } else {
            let b = res.unwrap().text().unwrap();
            println!("Recived Okay data {}", b);
            let v: Value  = serde_json::from_str(b.as_str()).unwrap();
            if let Value::String(url) = v[URL_KEY].clone() {
                wallpaper::set_from_url(url.as_str());
            }
        }
        thread::sleep(Duration::from_secs(WAIT_TIME_SEC));
    }
}