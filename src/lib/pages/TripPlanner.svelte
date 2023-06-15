<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { info, error } from "../../util";
    import ArrowRight from "svelte-bootstrap-icons/lib/ArrowRight.svelte";
    import LocationSearch from "../components/LocationSearch.svelte";
    import type { Plan } from "../../types/trip_planner";
    import { toPartialLocation } from "../../types/common";
    import type { Location } from "../../types/common";

    //TODO fix empty response body: Look at keys in request parameters!!
    async function search() {
        if (!start || !end) {
            alert("Please provide an origin and a destination!");
            return;
        }
        info("[TripPlanner] Fetching API response...");
        try {
            plans = await invoke("trip_planner", {
                origin: toPartialLocation(start),
                destination: toPartialLocation(end),
                date: (document.getElementById("date") as HTMLInputElement).value,
                time: (document.getElementById("time") as HTMLInputElement).value.split(":").map((x) => parseInt(x)),
                mode: (document.getElementById("time-type") as HTMLInputElement).value,
            });
        } catch (e) {
            error("[TripPlanner] Could not fetch API response!", e);
        }
    }

    async function keypress(event: KeyboardEvent) {
        if (event.key === "Enter") await search();
    }

    let now = new Date();
    let currentTime = `${now.getHours() < 10 ? "0" + now.getHours().toString() : now.getHours()}:${
        now.getMinutes() < 10 ? "0" + now.getMinutes().toString() : now.getMinutes()
    }`;
    let today = `${now.getFullYear()}-${now.getMonth() < 10 ? "0" + now.getMonth().toString() : now.getMonth()}-${
        now.getDay() < 10 ? "0" + now.getDay().toString() : now.getDay()
    }`;

    let start: Location | null = null;
    let end: Location | null = null;

    let plans: Plan[] = [];
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
            <input type="time" id="time" value={currentTime} min={currentTime} on:keypress={keypress} />
            <input type="date" id="date" value={today} min={today} on:keypress={keypress} />
        </div>

        <input type="button" id="reload" on:click={search} value="Go!" />
    </form>

    {#each plans as plan}
        <div>{plan}</div>
    {/each}
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

  .description
    color: #4d4d4c
    font-size: small
</style>
