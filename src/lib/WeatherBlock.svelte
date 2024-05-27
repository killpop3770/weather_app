<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import WeatherDisplayFragment from './WeatherDisplayFragment.svelte';
  import WeatherSearchFragment from './WeatherSearchFragment.svelte';

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
    })
    .catch((error) => console.error(error));
  }
</script>

<div id="main">
  <WeatherSearchFragment id="search_for_city" label_text="Search:" bind:addr={address}/>
  <br>
  <button on:click={request_weather_info}>Find weather!</button>
  <br><br><br>
  <WeatherDisplayFragment id="avg_temp" label_text="Avg temp:" temp_value={weather_data?.temp.toFixed(1)||'0.0'}/>
  <br>
  <WeatherDisplayFragment id="avg_temp_min" label_text="Min temp:" temp_value={weather_data?.temp_min.toFixed(1)||'0.0'}/>
  <br>
  <WeatherDisplayFragment id="avg_temp_max" label_text="Max temp:" temp_value={weather_data?.temp_max.toFixed(1)||'0.0'}/>
</div>

<style>
  #main {
    display: flex;
    flex-direction: column;
  }

  button {
    border-width: 1px;
    border-color: purple;
  }
</style>