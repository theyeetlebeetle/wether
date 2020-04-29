use std::fs::{File,OpenOptions};
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;

mod weather;
use weather::FutureResponse;

#[derive(Deserialize, Debug)]
enum Location {
    City(String),
    Zip(u8),
}

#[derive(Deserialize, Debug)]
struct Config {
    unit: String,
    loc: Vec<Location>,
    #[serde(default)]
    key: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            unit: String::from("imperial"),
            loc: vec!(Location::City(String::from("Seattle"))),
            key: String::from("secret"),
        }
    }
}

fn main() {
    let conf = load_conf();

    //let location_queries: Vec<String> = Vec::new();
    let responses: Vec<String> = Vec::new();
    let request_base = String::from("https://api.openweathermap.org/data/2.5/forecast?");

    for loc in conf.loc {
        let request_fmt = match loc {
            Location::City(c) => format!("{url}q={city}&appid={key}&units={unit}", 
                                      url=request_base, city=c, key=conf.key, unit=conf.unit),
            Location::Zip(z) => format!("{url}zip={zip}&appid={key}&units={unit}",
                                      url=request_base, zip=z, key=conf.key, unit=conf.unit),
        };
        //location_queries.push(request_fmt);
        responses.push( {
            let future_weather = reqwest::blocking::get(&request_fmt)
            .unwrap()
            .text()
            .unwrap();
            serde_json::from_str(&future_weather).unwrap()
        });
    }

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

fn load_conf() -> Config {
    let file = OpenOptions::new().read(true).write(true).create(true).open(
        ProjectDirs::from("org","theyeetlebeetle","wether").unwrap().config_dir()
        ).unwrap();
    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(conf) => conf,
        Err(err) => {
            println!("Could not read config file, using defaults: {}", err);
            Config::default()
        }
    }
}
