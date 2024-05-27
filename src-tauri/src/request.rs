use serde::{Serialize, Deserialize};
use owm_rs::owm_api::{get_city_coordinates, get_weather_by_coordinates};
use crate::config::Config;

#[derive(Serialize, Deserialize)]
pub struct WeatherRequestAnswer {
    temp: f32,
    temp_min: f32,
    temp_max: f32,
}

#[tauri::command]
pub async fn request_weather_info(geo: String) -> Result<WeatherRequestAnswer, String> {
    println!("Request weather info for: {}", &geo);

    let config = Config::new("open_weather_token.txt").expect("Can not make config!");

    let city_coords_result = get_city_coordinates(
        geo.clone(),
        config.get_token().to_string(),
    ).await;

    return match city_coords_result {
        Ok(coords) => {
            let weather_result = get_weather_by_coordinates(
                coords.get_latitude(),
                coords.get_longitude(),
                config.get_token().to_string(),
            ).await;

            match weather_result {
                Ok(weather) => {
                    let res = WeatherRequestAnswer {
                        temp: owm_rs::owm_utils::convert::kelvin_to_celsius(weather.main.temp),
                        temp_min: owm_rs::owm_utils::convert::kelvin_to_celsius(weather.main.temp_min),
                        temp_max: owm_rs::owm_utils::convert::kelvin_to_celsius(weather.main.temp_max),
                    };
                    Ok(res)
                }
                Err(err) => {
                    Err(format!("Error trying to receive weather data: {}", err))
                }
            }
        }
        Err(err) => {
            Err(format!("Error trying to receive coordinates: {}", err))
        }
    }
}