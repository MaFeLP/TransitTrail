<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { info, error } from "../../util";
    import ArrowRight from "svelte-bootstrap-icons/lib/ArrowRight.svelte";
    import LocationSearch from "../components/LocationSearch.svelte";

    function keypress(event: KeyboardEvent) {
        if (event.key === "Enter") {
            info("Pressed Enter");
        }
    }

    let now = new Date();
    let currentTime = `${now.getHours() < 10 ? "0" + now.getHours().toString() : now.getHours()}:${
        now.getMinutes() < 10 ? "0" + now.getMinutes().toString() : now.getMinutes()
    }`;

    let start: Location | null = null;
    let end: Location | null = null;
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
        </div>

        <input type="button" id="reload" on:click={() => alert("Reloading...")} value="Filter" />
    </form>
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
</style>
