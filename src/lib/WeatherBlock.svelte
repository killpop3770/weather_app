<script lang="ts">

import { invoke } from '@tauri-apps/api/tauri'

interface WeatherData {
  temp: number,
  temp_min: number,
  temp_max: number
}

  let address = "";
  let weather_data : WeatherData;

  async function request_weather_info() {
    invoke("request_weather_info", {geo: address})
    .then((result: any) => {
      console.log(result);
      weather_data = {
        temp: result.temp,
        temp_min: result.temp_min,
        temp_max: result.temp_max,
      };
    
      console.log(weather_data.temp);
    })
    .catch((error) => console.error(error));
  }
</script>

<div id="main">
  
  <div class="weather_element">
    <label for="search_input_for_city">Avg temp: </label>
    <input type="text" id="search_input_for_city" bind:value={address}/>
  </div>

  <button on:click={request_weather_info}>
    Find weather! 
  </button>

  <div class="weather_element">
    <label for="avg_temp">Avg temp: </label>
    <input type="text" id="avg_temp" value={weather_data?.temp.toFixed(1)||'0.0'}/>
  </div>
  <div class="weather_element">
    <label for="avg_temp_min">Min temp: </label>
    <input type="text" id="avg_temp_min" value={weather_data?.temp_min.toFixed(1)||'0.0'}/>
  </div>
  <div class="weather_element">
    <label for="avg_temp_max">Max temp: </label>
    <input type="text" id="avg_temp_max" value={weather_data?.temp_max.toFixed(1)||'0.0'}/>
  </div>
</div>

<style>
  
  #main{
    display: flex;
    flex-direction: column;
  }
  
  .weather_element {
    display:flex; 
    flex-direction: row; 
    justify-content: center; 
    align-items: center;
  }

</style>