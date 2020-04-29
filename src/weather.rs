use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Default)]
pub struct Coord {
    lon: f32,
    lat: f32
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

#[derive(Serialize, Deserialize)]
pub struct Weather {
    id: u16,
    pub main: String,
    pub description: String,
    icon: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: u32,
    pub humidity: u16,
    #[serde(default)]
    pub sea_level: u16,
    #[serde(default)]
    pub grnd_level: u16,
    #[serde(default)]
    pub temp_kf: f32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Wind {
    speed: f32,
    deg: f32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Clouds {
    all: u16,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Rain {
    #[serde(rename="1h")]
    one_hour: f32,
    #[serde(rename="3h")]
    three_hours: f32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Snow {
    #[serde(rename="1h")]
    one_hour: f32,
    #[serde(rename="3h")]
    three_hours: f32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Sys {
    #[serde(rename="type")]
    ttype: u32,
    #[serde(default)]
    id: u32,
    #[serde(default)]
    messagee: f32,
    country: String,
    sunrise: u64,
    sunset: u64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ListSys {
    #[serde(default)]
    id: u32,
    pod: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct City {
    #[serde(default)]
    id: u32,
    pub name: String,
    coord: Coord,
    pub country: String,
    #[serde(default)]
    pub population: u32,
    pub timezone: i32,
    #[serde(default)]
    pub sunrise: u32,
    #[serde(default)]
    pub sunset: u32 
}

#[derive(Serialize, Deserialize, Default)]
pub struct List {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub coord: Coord,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub timezone: i32,
    #[serde(default)]
    dtpub : u32,
    #[serde(default)]
    pub main: Main,
    #[serde(default)]
    pub weather: Vec<Weather>,
    #[serde(default)]
    pub clouds: Clouds,
    #[serde(default)]
    pub wind: Wind,
    #[serde(default)]
    pub sys: ListSys,
    #[serde(default)]
    pub dt_txt: String,     //"2020-01-07 15:00:00"
}

#[derive(Serialize, Deserialize)]
pub struct FutureResponse{
    pub cod: Value,
    pub message: u32,
    pub city: City,
    pub cnt: u32,
    pub list: Vec<List>,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherResponse {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    #[serde(default)]
    pub wind: Wind,
    #[serde(default)]
    pub clouds: Clouds,
    #[serde(default)]
    pub rain: Rain,
    #[serde(default)]
    pub snow: Snow,
    pub dt: u64,
    pub sys: Sys,
    pub timezone: i32,
    pub id: u32,
    pub name: String,
    pub cod: u32 
}
