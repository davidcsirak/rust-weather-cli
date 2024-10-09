use std::io;
use serde::Deserialize;
use colored::*;

// Struct to deserialize the JSON from API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct for weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String
}

// Struct for main weather info
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct for wind info
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64
}

// Get weather info from API
fn get_weather_info(city: String, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!("https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}", city, api_key);
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// Display weather info
fn display_weather_info(response: &WeatherResponse) {
    // Extract weather info
    let description= &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    let weather_text: String = format!(
        "Weather in {}: {} {},\n\
        > Temperature: {}\n\
        > Humidity: {}\n\
        > Pressure: {}\n\
        > Wind speed: {}",
        response.name,
        format!("{}", description).cyan(),
        get_temp_emoji(temperature),
        format!("{:.1}°C", temperature).cyan(),
        format!("{:.1}%", humidity).cyan(),
        format!("{:.1} hPa", pressure).cyan(),
        format!("{:.1} m/s", wind_speed).cyan()
    );

    println!("{}", weather_text.bright_yellow());
}

// Get emoji based on temp
fn get_temp_emoji(temperature: f64) -> &'static str {
    match temperature {
        t if t < 0.0 => "❄️",
        t if t < 10.0 => "☁️",
        t if t < 20.0 =>  "⛅",
        t if t < 30.0 =>  "️🌤️",
        _ =>  "🔥",
    }
}

fn get_user_input(prompt: &str) -> Result<String, io::Error> {
    println!("{}", prompt.bright_green());
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() {
    println!("{}", "Welcome to my Weather CLI!");
    loop {
        let api_key = "4acf5788c9e77dc936c3377500ba195c";

        let city = match get_user_input("Enter the name of the city:") {
            Ok(input) => input,
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        };

        match get_weather_info(city, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(_e) => {
                eprintln!("Error: No city found");
            }
        }

        let answer = match get_user_input("Do you want to search another city? (y/n):") {
            Ok(input) => input.to_lowercase(),
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        };

        if answer != "y" {
            break;
        }
    }
}
