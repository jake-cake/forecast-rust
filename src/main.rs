use error_chain::error_chain;
use std::io::Read;
use structopt::StructOpt;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
// use std::ops::Try;


error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(StructOpt)]
struct Cli {
    // input city
    city: String,
}

//structuring api info
#[derive(Serialize, Deserialize, Debug)] 
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
    //  message: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details
}
#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

//converting api to json
fn fetch_data(city:&str) -> Result<Forecast>{
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid=5f1959906bc793e97f7b81110c8a8d38", city);
    let res = reqwest::blocking::get(&url)?;
    let json_responce = res.json::<Forecast>().unwrap();

    // let json_response: Result<(), Error> = try {
    //     res.json::<Forecast>().unwrap()?;
    //   };
  
    //   if let Err(e) = json_response {
    //       println!("Please Enter valid city name.");
    //   }

    Ok(json_responce)
}

// kelvit to celcius converter
fn kelvin_to_celcius(kel: f64) -> f64{
    kel - 273.15
}
// mps to kmh conventer
fn miles_per_sec_to_kmh(inputspeed: f64) -> f64 {
    inputspeed * 3.6
}

fn main(){
    // prepairing variables to run 
    let args = Cli::from_args();
    let city = &args.city[..];
    let sum_data = fetch_data(city).unwrap();
    let temp_cel = kelvin_to_celcius(sum_data.main.temp);
    let feels_like_cel = kelvin_to_celcius(sum_data.main.feels_like);
    let wind_speed_kmh = miles_per_sec_to_kmh(sum_data.wind.speed);
    // printing out the result
    println!("\nCity: {}.",sum_data.name);
    println!("Weather: {}. \nTemperature: {} ℃ . \nFeels Like: {} ℃ . \nWind speed: {} km/h.\n",sum_data.weather.details.description,temp_cel.round(),feels_like_cel.round(),wind_speed_kmh.round());
} 