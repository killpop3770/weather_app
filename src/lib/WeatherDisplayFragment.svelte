<script lang="ts">
  
    export let id: string;
    export let label_text: string;
    export let temp_value: string | Number;

</script>
  
<div class="weather_element">
    <label for={id}>{label_text}</label>
    <input type="text" id={id} bind:value={temp_value} readonly/>
</div>
  
<style>
    .weather_element {
        display:flex; 
        flex-direction: row; 
        justify-content: center; 
        align-items: center;
    }

    .weather_element label {
        margin-right: 20px;
    }
</style>
  