use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;

const API_KEY: &str = "secret";

mod weather;
use weather::FutureResponse;

#[derive(Deserialize, Debug)]
struct Config {
    unit: String,
    city: String,
    #[serde(default)]
    country: String,
}

fn main() {
    let file = OpenOptions::new().read(true).write(true).create(true).open(
        ProjectDirs::from("org","theyeetlebeetle","wether").unwrap().config_dir()
        ).unwrap();

    let conf: Config;

    let request_future =
        format!("https://api.openweathermap.org/data/2.5/forecast?q={city}&appid={key}&units={unit}",
                                  city = "Seattle",
                                  key = API_KEY, //TODO: refactor API_KEY into config
                                  unit = "imperial");

    let future_weather = reqwest::blocking::get(&request_future)
        .unwrap()
        .text()
        .unwrap();
    let future: FutureResponse = serde_json::from_str(&future_weather).unwrap();

    println!("Current temp: \t{:.0}", future.list[0].main.temp);
    println!("High/Low: \t{:.0}/{:.0}", future.list[0].main.temp_max, future.list[0].main.temp_min);
    println!("Looks like: \t{} || {}", future.list[0].weather[0].main,
             future.list[0].weather[0].description);
}
