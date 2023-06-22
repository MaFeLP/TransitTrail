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
    import { info, error } from "../../util";
    import ArrowRight from "svelte-bootstrap-icons/lib/ArrowRight.svelte";
    import LocationSearch from "../components/LocationSearch.svelte";
    import type { Plan } from "../../types/trip_planner";
    import { toPartialLocation } from "../../types/common";
    import type { Location } from "../../types/common";
    import TransitPlan from "../components/TransitPlan.svelte";
    import GoogleTransitPlan from "../components/GoogleTransitPlan.svelte";

    async function search() {
        if (!start || !end) {
            alert("Please provide an origin and a destination!");
            return;
        }
        info("[TripPlanner] Fetching API response...");
        let options = {
            mode: (document.getElementById("time-type") as HTMLInputElement).value,
        };

        if ((document.getElementById("date") as HTMLInputElement).value) {
            options["date"] = (document.getElementById("date") as HTMLInputElement).value;
        }
        if ((document.getElementById("time") as HTMLInputElement).value) {
            options["time"] = (document.getElementById("time") as HTMLInputElement).value
                .split(":")
                .map((x) => parseInt(x));
        }

        try {
            plans = await invoke("trip_planner", {
                origin: toPartialLocation(start),
                destination: toPartialLocation(end),
                ...options,
            });
            google_plans = await invoke("google_trip_planner", {
                origin: toPartialLocation(start, false),
                destination: toPartialLocation(end, false),
            });
        } catch (e) {
            error("[TripPlanner] Could not fetch API response!", e);
        }
    }

    async function keypress(event: KeyboardEvent) {
        if (event.key === "Enter") await search();
    }

    let start: Location | null = null;
    let end: Location | null = null;

    let plans: Plan[] = [];
    let google_plans: Plan[] = [];
</script>

<div>
    <h2>Trip Planner</h2>
    <form>
        <div class="filter">
            <LocationSearch
                setLocation={(location) => {
                    // setLocation gives a Location as a parameter, so we can safely assign here
                    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
                    start = location;
                }}
                placeholder={"Select Start"}
                id="search-location-start"
            />
            <span class="bold"><ArrowRight /></span>
            <LocationSearch
                setLocation={(location) => {
                    // setLocation gives a Location as a parameter, so we can safely assign here
                    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
                    end = location;
                }}
                placeholder={"Select Destination"}
                id="search-location-end"
            />
        </div>

        <div class="filter left-separator">
            <select name="TimeType" id="time-type">
                <option value="DepartBefore">Depart Before</option>
                <option value="DepartAfter" selected>Depart After</option>
                <option value="ArriveBefore">Arrive Before</option>
                <option value="ArriveAfter">Arrive After</option>
            </select>
            <input type="time" id="time" on:keypress={keypress} />
            <input type="date" id="date" on:keypress={keypress} />
        </div>

        <input type="button" id="reload" on:click={search} value="Go!" />
    </form>

    <div id="plans">
        <div id="transit-plans">
            {#if plans.length !== 0}
                <h4>NaviGo</h4>
            {/if}
            {#each plans as plan, index}
                <TransitPlan {plan} id="transit-plan-{index}" />
            {/each}
        </div>
        <div id="google-plans">
            {#if google_plans.length !== 0}
                <h4>Google Maps</h4>
            {/if}
            {#each google_plans as plan, index}
                <GoogleTransitPlan {plan} id="google-transit-plan-{index}" />
            {/each}
        </div>
    </div>
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

  div.filter
    display: flex
    flex-direction: row
    align-items: center
    justify-content: center
    gap: 10px

  .left-separator
    border-left: var(--fg) solid 1px
    padding-left: 25px

  .bold
    font-weight: bold

  div#plans
    display: flex
    flex-direction: row
    align-items: start
    justify-content: space-evenly
    width: 100%
    margin-top: 20px

  div#transit-plans
    display: flex
    flex-direction: column
    align-items: start
    justify-content: center
    gap: 10px
</style>
