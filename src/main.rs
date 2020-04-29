use std::fs::OpenOptions;
use std::io::{BufWriter, BufReader};
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;
use clap::Clap;

mod weather;
use weather::FutureResponse;

/// A really bad cli app to check weather in one or more cities
#[derive(Clap)]
#[clap(version = "0.1", author = "yeetlebeetle")]
struct Opts {
    /// Adds a city to the config file and saves it
    #[clap(short = "a", long = "add")]
    add: Option<String>,
    /// Removes a city from the config file and saves it 
    #[clap(short = "r", long = "remove")]
    remove: Option<String>,
    /// Check the weather of a single city
    city: Option<String>,
    //#[clap(subcommand)]
    //subcmd: SubCommand,
}

#[derive(Serialize, Deserialize, Debug)]
enum Location {
    City(String),
    Zip(u8),
}

#[derive(Serialize, Deserialize, Debug)]
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
    let opts: Opts = Opts::parse();

    let conf = load_conf();

    let mut responses: Vec<FutureResponse> = Vec::new();
    let request_base = String::from("https://api.openweathermap.org/data/2.5/forecast?");

    for loc in &conf.loc {
        let request_fmt = match loc {
            Location::City(c) => format!("{url}q={city}&appid={key}&units={unit}", 
                                      url=request_base, city=c, key=&conf.key, unit=&conf.unit),
            Location::Zip(z) => format!("{url}zip={zip}&appid={key}&units={unit}",
                                      url=request_base, zip=z, key=&conf.key, unit=&conf.unit),
        };

        let future_weather = reqwest::blocking::get(&request_fmt)
            .unwrap()
            .text()
            .unwrap();

        let test: Result<Eror, serde_json::error::Error> =
            serde_json::from_str(&future_weather);
        match test {
            Ok(err) => {
                println!("There was an issue: \n{:?}", err);
                continue;
            }
            Err(_) => {},
        }

        let response = serde_json::from_str(&future_weather);
        match response {
            Ok(value) => responses.push(value),
            Err(err) => println!("There was an issue: {:?}", err),
        }
    }

    for each in responses {
        println!("{}", each.city.name);
        println!("Current temp: \t{:.0}", each.list[0].main.temp);
        println!("High/Low: \t{:.0}/{:.0}", each.list[0].main.temp_max, each.list[0].main.temp_min);
        println!("Looks like: \t{} || {}", each.list[0].weather[0].main,
                 each.list[0].weather[0].description);
    }
}

fn save_conf(conf: &Config) -> Result<(),Box<dyn std::error::Error>>{
    let file = OpenOptions::new().write(true).create(true).open(
        ProjectDirs::from("org","theyeetlebeetle","wether").unwrap().config_dir()
        )?;

    let writer = BufWriter::new(file);
    match serde_json::to_writer(writer, &conf) {
        Ok(_) => {
            println!("Saved config!");
            Ok(())
        }
        Err(err) => {
            println!("Failed to save config {:?}", err);
            Err(Box::new(err))
        }
    }
}

fn load_conf() -> Config {
    let file = match OpenOptions::new().write(true).create(true).read(true).open(
        ProjectDirs::from("org","theyeetlebeetle","wether").unwrap().config_dir()
        ) {
        Ok(f) => f,
        Err(err) => {
            println!("Could not open config: {:?}", err.kind());
            panic!();
        },
    };
    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(conf) => conf,
        Err(err) => {
            println!("Could not read config file, using defaults: {}", err);
            let conf = Config::default();
            let _ = save_conf(&conf);
            conf
        }
    }
}
