<script lang="ts">
  import WeatherBlock from './lib/WeatherBlock.svelte'

</script>

<main>
  <h1>City weather</h1>
  <br><br>
  <div class="card">
    <WeatherBlock />
  </div>

</main>

<style>
  main {
    display: flex;
    flex-direction: column;
  }
</style>
