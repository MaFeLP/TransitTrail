<script lang="js">
    import {invoke} from "@tauri-apps/api/tauri";

    function showStops() {
        const stopNumber = document.getElementById("stop-number").value;
        invoke("stop_schedule", {stop: parseInt(stopNumber)})
            .then(function (data) {
                const stops = document.getElementById("stops");
                stops.innerHTML = data;

            }).catch(
            function (error) {
                console.error(error);
            }
        )
    }


</script>

<div>
    <h1>Stops</h1>
    <input type="number" placeholder="Stop Number" id="stop-number">
    <button on:click={showStops}>Show Stops</button>
    <div id="stops"></div>
</div>

<style>

</style>
