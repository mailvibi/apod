use reqwest;
use std::{fs::File, path::PathBuf};
use serde_json::{Value};
use std::{thread, time::Duration};
use process_path::get_executable_path;
use wallpaper;

fn main() {
    const APOD_URL : &str = "https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY";
    const URL_KEY: &str = "hdurl";
    const WAIT_TIME_SEC : u64 = 60 * 60 * 5;
    const IMAGEFILE : &str = "imgfile.jpg";
    let path : PathBuf; 
    if let Some(p) = get_executable_path() {
        path = p;
    } else {
        panic!("");
    };

    let imgfile = path.parent().unwrap().join(IMAGEFILE);
    println!("{:?}", imgfile);
    loop {
        let mut oldurl : String;
        let res = reqwest::blocking::get(APOD_URL);
        if let Err(e) = res {
            println!("Received error : {}", e);
        } else {
            let b = res.unwrap().text().unwrap();
            println!("Recived Okay data {}", b);
            let v: Value  = serde_json::from_str(b.as_str()).unwrap();
            if v[URL_KEY] != Value::Null {
                println!("{:?}", v[URL_KEY]);
            }
            let res = reqwest::blocking::get(v[URL_KEY].as_str().unwrap());
            if let Err(e) = res {
                println!("Received error {}", e);
            } else {
                println!("received file {} - > res {:?}", v[URL_KEY], res);
                {
                    let mut ifile = File::create(imgfile.clone()).unwrap();
                    res.unwrap().copy_to(&mut ifile).unwrap();
                }
                wallpaper::set_from_path(imgfile.to_str().unwrap()).unwrap();
            }
        }
        thread::sleep(Duration::from_secs(WAIT_TIME_SEC));
    }
}