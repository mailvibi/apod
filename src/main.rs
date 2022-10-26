use reqwest;
use serde_json::{Value};
use wallpaper;


fn main() -> Result<(), &'static str> {
    const APOD_URL : &str = "https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY";
    const URL_KEY: &str = "hdurl";

    let body = reqwest::blocking::get(APOD_URL)
                                                .or(Err("Getting URL failed"))?
                                                .text()
                                                .or(Err("Getting Body of response failed"))?;
    let jsnbdy: Value = serde_json::from_str(body.as_str()).or(Err("json parsing failed"))?;

    if let Value::String(url) = jsnbdy[URL_KEY].clone() {
        wallpaper::set_from_url(url.as_str()).or(Err("unable to set wallpaper"))?;
    } else {
        return Err("No hdurl key found in json response");
    }
    Ok(())
}