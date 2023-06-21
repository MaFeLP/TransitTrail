<!-- TransitTrail - Navigate Winnipeg Transit with a different style
   Copyright (C) - 2023 Foxx Azalea Pinkerton, Max Fehlinger

   This program is free software: you can redistribute it and/or modify it under
   the terms of the GNU General Public License as published by the Free Software
   Foundation, either version 3 of the License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but WITHOUT
   ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
   FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

   You should have received a copy of the GNU General Public License along with
   this program. If not, see <https://www.gnu.org/licenses/>. -->


<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { error } from "../../util";
    import Filter from "svelte-bootstrap-icons/lib/Filter.svelte";
    import type { Stop } from "../../types/filters";

    async function showStops() {
        document.getElementById("stops").innerHTML = `<small>Loading...</small>`;

        let stopNumber = (document.getElementById("stop-number") as HTMLInputElement).value;
        let start = (document.getElementById("start") as HTMLInputElement).value;
        let end = (document.getElementById("end") as HTMLInputElement).value;
        let limit = parseInt((document.getElementById("limit") as HTMLInputElement).value);

        let filters: Stop[] = [];
        if (start) filters.push({ Start: start.split(":").map((x) => parseInt(x)) });
        if (end) filters.push({ End: end.split(":").map((x) => parseInt(x)) });
        if (isFinite(limit)) filters.push({ MaxResultsPerRoute: limit });

        try {
            let schedule = await invoke("stop_schedule", { stop: parseInt(stopNumber), filter: filters });
            document.getElementById("stops").innerHTML = schedule.toString();
        } catch (err) {
            error(`Error getting stop schedule ${err}`, err);
        }
    }

    async function keypress(event: KeyboardEvent) {
        if (event.key === "Enter") {
            await showStops();
        }
    }

    // Set default filters
    const twoHoursMS = 2 * 60 * 60 * 1000;

    let now = new Date();
    let start = `${now.getHours() < 10 ? "0" + now.getHours().toString() : now.getHours()}:${
        now.getMinutes() < 10 ? "0" + now.getMinutes().toString() : now.getMinutes()
    }`;

    now.setTime(now.getTime() + twoHoursMS);
    let end = `${now.getHours() < 10 ? "0" + now.getHours().toString() : now.getHours()}:${
        now.getMinutes() < 10 ? "0" + now.getMinutes().toString() : now.getMinutes()
    }`;
</script>

<div>
    <h1>Stops</h1>
    <form>
        <div class="filter">
            <Filter />
            <input type="number" placeholder="Stop Number" id="stop-number" tabindex="0" on:keypress={keypress} />
        </div>

        <div class="filter left-separator">
            <label for="start">Time Range:</label>
            <input type="time" id="start" value={start} min={start} on:keypress={keypress} />
            <label for="end">To</label>
            <input type="time" id="end" value={end} min={start} on:keypress={keypress} />
        </div>

        <div class="filter left-separator">
            <input type="number" placeholder="Limit per Route" id="limit" on:keypress={keypress} />
        </div>

        <div class="filter left-separator">
            <input type="button" on:click={showStops} value="Show Schedule" />
        </div>
    </form>
    <div id="stops" />
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
