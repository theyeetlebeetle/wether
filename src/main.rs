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

#[derive(Deserialize, Debug)]
struct Eror {
    cod: u16,
    message: String,
}

fn main() {
    let conf = load_conf();

    let mut responses: Vec<FutureResponse> = Vec::new();
    let request_base = String::from("https://api.openweathermap.org/data/2.5/forecast?");

    for loc in conf.loc {
        let request_fmt = match loc {
            Location::City(c) => format!("{url}q={city}&appid={key}&units={unit}", 
                                      url=request_base, city=c, key=conf.key, unit=conf.unit),
            Location::Zip(z) => format!("{url}zip={zip}&appid={key}&units={unit}",
                                      url=request_base, zip=z, key=conf.key, unit=conf.unit),
        };

        let future_weather = reqwest::blocking::get(&request_fmt)
            .unwrap()
            .text()
            .unwrap();

        println!("Beetlejuice");

        let test: Result<Eror, serde_json::error::Error> =
            serde_json::from_str(&future_weather);
        match test {
            Ok(err) => {
                println!("There was an issue: \n{:?}", err);
                continue;
            }
            Err(value) => {},
        }

        let response = serde_json::from_str(&future_weather);
        match response {
            Ok(value) => responses.push(value),
            Err(err) => println!("There was an issue: {:?}", err),
        }

        //let response_container: Collective = serde_json::from_str(&future_weather).unwrap();
        //responses.push(response_container);

        //serde_json::from_str(&future_weather).unwrap();
    }

    //println!("Current temp: \t{:.0}", future.list[0].main.temp);
    //println!("High/Low: \t{:.0}/{:.0}", future.list[0].main.temp_max, future.list[0].main.temp_min);
    //println!("Looks like: \t{} || {}", future.list[0].weather[0].main,
    //         future.list[0].weather[0].description);
}

//fn printer() {
//    match &message {
//        Eror => println!("There was an issue:\n{:?}", message),
//        FutureResponse => println!("It worked out"),
//    }
//}

fn load_conf() -> Config {
    let file = match OpenOptions::new().read(true).open(
        ProjectDirs::from("org","theyeetlebeetle","wether").unwrap().config_dir()
        ) {
        Ok(f) => f,
        Err(err) => {
            println!("Could open config: {:?}", err.kind());
            panic!();
        }
    };
    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(conf) => conf,
        Err(err) => {
            println!("Could not read config file, using defaults: {}", err);
            Config::default()
        }
    }
}
