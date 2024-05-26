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
<input type="text" id="search_input_for_city" bind:value={address}/>
<button on:click={request_weather_info}>
  Find weather! 
</button>
<input type="text" id="avg_temp" value={weather_data?.temp}/>
<input type="text" id="avg_temp_min" value={weather_data?.temp_min}/>
<input type="text" id="avg_temp_max" value={weather_data?.temp_max}/>
</div>

<style>
  #main{
  display: flex;
  flex-direction: column;
  }
</style>