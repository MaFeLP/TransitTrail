<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { error, info } from "tauri-plugin-log-api";
    import Filter from "svelte-bootstrap-icons/lib/Filter.svelte"

    function showStops() {
        document.getElementById("stops").innerHTML = `<small>loading...</small>`;

        let stopNumber = (document.getElementById("stop-number") as HTMLInputElement).value;
        let start = (document.getElementById("start") as HTMLInputElement).value;
        let end = (document.getElementById("end") as HTMLInputElement).value;
        let limit = parseInt((document.getElementById("limit") as HTMLInputElement).value);

        let filters: any[] = [];
        if (start) filters.push({ Start: start.split(":").map(x => parseInt(x)) });
        if (end) filters.push({ End: end.split(":").map(x => parseInt(x)) });
        if (isFinite(limit)) filters.push({ MaxResultsPerRoute: limit });

        invoke("stop_schedule", { stop: parseInt(stopNumber), filter: filters })
            .then(function (data) {
                document.getElementById("stops").innerHTML = data.toString();
            }).catch(async (err) => {
                await error(`Error getting stop schedule ${err}`);
                console.error(err);
            }
        )
    }

    function keypress(event) {
        if (event.key === "Enter") {
            showStops();
        }
    }

    // Set default filters
    const twoHoursMS = 2 * 60 * 60 * 1000;

    let now = new Date();
    let start = `${now.getHours()}:${now.getMinutes()}`;
    if (start.length === 4) start = "0" + start;

    now.setTime(now.getTime() + twoHoursMS);
    let end = `${now.getHours()}:${now.getMinutes()}`
    if (end.length === 4) end = "0" + end;
</script>

<div>
    <h1>Stops</h1>
    <form>
        <div class="filter">
            <Filter />
            <input type="number" placeholder="Stop Number" id="stop-number" autofocus on:keypress={keypress}>
        </div>

        <div class="filter left-separator">
            <label for="start">Time Range:</label>
            <input type="time" id="start" value="{start}" min="{start}" on:keypress={{keypress}}>
            <label for="end">To</label>
            <input type="time" id="end" value="{end}" min="{start}" on:keypress={{keypress}}>
        </div>


        <div class="filter left-separator">
            <input type="number" placeholder="Limit per Route" id="limit" on:keypress={keypress}>
        </div>

        <div class="filter left-separator">
            <input type="button" on:click={showStops} value="Show Schedule">
        </div>
    </form>
    <div id="stops"></div>
</div>

<style lang="sass">
  @use "src/styles/variables" as var

  form
    display: flex
    flex-direction: row
    align-items: center
    justify-content: start
    margin: 0 auto
    width: 100%
    gap: 25px
    padding-bottom: 20px
    border-bottom: 1px solid var(--fg)

  #start, #end
    height: var.$line-height

  div.filter
    display: flex
    flex-direction: row
    align-items: center
    justify-content: center
    gap: 10px

  .left-separator
    border-left: var(--fg) solid 1px
    padding-left: 25px
</style>
