extern crate reqwest;
extern crate serde_json;
extern crate winapi;

use winapi::um::winuser::{SystemParametersInfoA, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER};

use std::{fs::File, path::PathBuf};
use std::io::BufWriter;
use serde_json::{Value};
use std::{thread, time::Duration};
use process_path::get_executable_path;

use std::ffi::{CString, c_void};
fn setwallpaper(wallpaper : &str) {
    println!("setting wallpaper {}", wallpaper);
    let image_path = CString::new(wallpaper).unwrap();
    unsafe {
        SystemParametersInfoA(SPI_SETDESKWALLPAPER, 0, 
            image_path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE);
    }
}



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
    println!("{:?}", path.parent());
    let imgfile = path.parent().unwrap().join(IMAGEFILE);
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
                let file = File::options().write(true).open(imgfile.clone()).unwrap();
                let mut writer = BufWriter::new(file);
                if let e = res.unwrap().copy_to(&mut writer) {
                    setwallpaper(imgfile.to_str().unwrap());
                }
            }
        }
        thread::sleep(Duration::from_secs(WAIT_TIME_SEC));
    }
}