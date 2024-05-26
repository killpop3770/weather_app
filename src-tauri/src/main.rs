// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use owm_rs::owm_api::{get_city_coordinates, get_weather_by_coordinates};

mod config;

#[derive(Serialize, Deserialize)]
struct WeatherRequestAnswer {
    temp: f32,
    temp_min: f32,
    temp_max: f32,
}

#[tauri::command]
async fn request_weather_info(geo: String) -> Result<WeatherRequestAnswer, String> {
    println!("Request weather info for: {}", &geo);

    //TODO: Как он находит города ?
    //TODO: Как улучшить код на раст ?
    //TODO: Как вывести значения градусов на экран ?
    //TODO: Стоит ли делать мини настройки для API ?

    let city_coords_result = get_city_coordinates(
        geo.clone(),
        config::OWM_API_KEY.to_string()
    ).await;

    match city_coords_result {
        Ok(coords) => {
            let weather_result = get_weather_by_coordinates(
                coords.get_latitude(),
                coords.get_longitude(),
                config::OWM_API_KEY.to_string(),
            ).await;

            match weather_result {
                Ok(weather) => {
                    let res = WeatherRequestAnswer {
                        temp: owm_rs::owm_utils::convert::kelvin_to_celsius(weather.main.temp),
                        temp_min: owm_rs::owm_utils::convert::kelvin_to_celsius(weather.main.temp_min),
                        temp_max: owm_rs::owm_utils::convert::kelvin_to_celsius(weather.main.temp_max),
                    };
                    return Ok(res);
                }
                Err(err) => {
                    return Err(format!("Error trying to receive weather data: {}", err));
                }
            }
        }
        Err(err) => {
            return Err(format!("Error trying to receive coordinates: {}", err));
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![request_weather_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application!");
}
