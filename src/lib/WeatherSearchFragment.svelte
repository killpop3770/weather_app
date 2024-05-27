<script lang="ts">

    export let id: string;
    export let label_text: string;
    export let addr: string;

</script>

<div class="weather_search_element">
    <label for={id}>{label_text}</label>
    <input type="text" id={id} bind:value={addr}/>
</div>

<style>
    .weather_search_element {
        display:flex; 
        flex-direction: row; 
        justify-content: center; 
        align-items: center;
        
    }

    .weather_search_element label {
        margin-right: 20px;
    }

    .weather_search_element input {
        width: 100%;
    }

</style>
  