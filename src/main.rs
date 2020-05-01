use std::fs::OpenOptions;
use std::io::{Write, BufWriter, BufReader};
use serde::{Serialize, Deserialize};
use reqwest::Client;
use directories::ProjectDirs;
use clap::Clap;
use tokio;

mod weather;
use weather::FutureResponse;

/// A really bad cli app to check weather in one or more cities. 
/// 
/// You must have an api key from OpenWeatherMap in your config file 
#[derive(Clap)]
#[clap(version = "0.1", author = "yeetlebeetle")]
struct Opts {
    /// Adds a city to the config file and saves it
    #[clap(short = "a", long = "add", value_name="city")]
    add: Option<String>,
    /// Removes a city from the config file and saves it 
    #[clap(short = "r", long = "remove", value_name="city")]
    remove: Option<String>,
    /// Check the weather of a single city
    city: Option<String>,
    //#[clap(subcommand)]
    //subcmd: SubCommand,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Location {
    City(String),
    Zip(u16),
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let mut conf = load_conf();

    if let Some(new_loc) = opts.add {
        conf.loc.push( match new_loc.parse::<u16>() {
            Ok(t) => Location::Zip(t),
            Err(_) => Location::City(new_loc),
        });
        let _ = save_conf(&conf);
    };

    if let Some(rem_loc) = opts.remove {
        match check_city(&conf, match rem_loc.parse::<u16>() {
            Ok(t) => Location::Zip(t),
            Err(_) => Location::City(rem_loc),
        }) {
            Some(i) => {
                conf.loc.remove(i);
                let _ = save_conf(&conf);
            },
            None => {
                println!("Location not in config file");
            },
        }
    };

    let client = Client::new();
    if let Some(loc) = opts.city {
        check_weather(&conf, &client, &match loc.parse::<u16>() {
            Ok(t) => Location::Zip(t),
            Err(_) => Location::City(loc),
        })
        .await
        .unwrap();
    } else {
        for loc in &conf.loc {
            check_weather(&conf, &client, loc)
                .await
                .unwrap();
        }
    }
    Ok(())
}

fn check_city(conf: &Config, needle: Location) -> Option<usize> {
    for (index,loc) in conf.loc.iter().enumerate() {
        if *loc == needle {
            return Some(index);
        }
    }
    None
}

async fn check_weather(conf: &Config, client: &Client, loc: &Location) -> Result<(), reqwest::Error> {
    let request_base = String::from("https://api.openweathermap.org/data/2.5/forecast?");

    let request_fmt = match loc {
        Location::City(c) => format!("{url}q={city}&appid={key}&units={unit}", 
                                  url=request_base, city=c, key=&conf.key, unit=&conf.unit),
        Location::Zip(z) => format!("{url}zip={zip}&appid={key}&units={unit}",
                                  url=request_base, zip=z, key=&conf.key, unit=&conf.unit),
    };

    let future_weather = client.get(&request_fmt)
        .send()
        .await?
        .text()
        .await?;

    let test: Result<Eror, serde_json::error::Error> =
        serde_json::from_str(&future_weather);
    match test {
        Ok(err) => {
            println!("There was an issue: \n{:?}", err);
            return Ok(()); //look I really don't know how to handle these errors don't judge me
        }
        Err(_) => {},
    }

    let response: FutureResponse = match serde_json::from_str(&future_weather) {
        Ok(value) => value,
        Err(err) => {
            println!("There was an issue: {:?}", err);
            return Ok(());
        }
    };

    println!("===========================================");
    println!("{}, {}", response.city.name, response.city.country);
    println!("Current temp: \t{:.0}", response.list[0].main.temp);
    println!("High/Low: \t{:.0}/{:.0}", response.list[0].main.temp_max, response.list[0].main.temp_min);
    println!("Looks like: \t{} || {}", response.list[0].weather[0].main,
             response.list[0].weather[0].description);
    Ok(())
}

fn save_conf(conf: &Config) -> Result<(),Box<dyn std::error::Error>>{
    let file = OpenOptions::new().write(true).create(true).open(
        ProjectDirs::from("org","theyeetlebeetle","wether").unwrap().config_dir()
        )?;

    let mut writer = BufWriter::new(&file);
    let out_file = serde_json::to_string(&conf)?;
    writer.write_all(out_file.as_bytes())?;
    file.set_len(out_file.len() as u64)?;
    println!("Saved config!");
    Ok(())
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
