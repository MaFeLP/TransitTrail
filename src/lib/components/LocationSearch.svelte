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
    import { error, info } from "../../util";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Location } from "../../types/common";
    import { LocationType } from "../../types/common";
    import type { Settings } from "../../types/settings";
    import { onMount } from "svelte";
    import type { Stop } from "../../types/stops";

    export let setLocation: (Location) => void;
    export let placeholder: string;
    export let id: string;

    let dialog: HTMLDialogElement | null = null;
    let interval: number | null = null;
    let oldSearchInput = "";
    let searchResults: Location[] = [];

    onMount(() => {
        dialog = document.getElementById(id) as HTMLDialogElement;
    });

    function submit(index: number) {
        let selected = searchResults[index];
        switch (selected.type) {
            case LocationType.Address:
                placeholder = `${selected["street-number"]} ${selected.street.name}`;
                break;
            case LocationType.Monument:
                placeholder = selected.name;
                break;
            case LocationType.Intersection:
                placeholder = `${selected.street.name} @ ${selected["cross-street"].name}`;
                break;
            case LocationType.Point:
                placeholder = `(${selected.centre.latitude}, ${selected.centre.longitude})`;
                break;
        }
        setLocation(selected);
        searchResults = [];
        dialog.close(placeholder);
    }

    function search() {
        let query = (document.getElementById(`${id}-query`) as HTMLInputElement).value;
        if (query === oldSearchInput) return;
        info(`[LocationSearch]: Querying for locations: ${query}`);
        invoke("search_locations", { input: query })
            .then((res: Location[]) => {
                searchResults = res;
                console.info("[LocationSearch]: Got search results", res);
            })
            .catch((err) => {
                error(`Could not get search results ${err}`, err);
            });
    }

    async function openDialog() {
        dialog.showModal();

        let settings: Settings = await invoke("get_settings");

        info("[LocationSearch]: Auto Search Interval running");
        if (!interval) interval = setInterval(search, settings.search_interval);
    }

    function clear() {
        info("[LocationSearch]: clearing interval");
        if (interval) clearInterval(interval);
        interval = null;
    }

    function keypress(event: KeyboardEvent) {
        if (event.key === "Enter") {
            search();
            return;
        }
        if (event.key === "Escape") {
            dialog.close();
            return;
        }
    }

    async function stopNumber() {
        let stopNumber = NaN;
        let message = "Enter a stop number";

        // eslint-disable-next-line no-constant-condition
        while (true) {
            let input = prompt(message);
            if (input === null) return;
            stopNumber = parseInt(input);
            if (isNaN(stopNumber)) {
                message = "Invalid stop number! Please try again!";
                continue;
            }

            try {
                let stop: Stop = await invoke("stop_info", { id: stopNumber });
                placeholder = `#${stop.key} (${stop.name})`;
                setLocation({ type: LocationType.Stop, key: stopNumber });
                dialog.close();
                break;
            } catch (err) {
                error(`Could not get stop ${stopNumber}`, err);
                message = "Invalid stop number! Please try again!";
            }
        }
    }
</script>

<dialog class="search-dialog" {id} on:close={clear} on:keypress={keypress}>
    <div class="contents">
        <input type="button" value="Enter a stop number" id="{id}-stop-number" on:click={stopNumber} />
        <p class="or-label"><b>OR</b></p>
        <input type="text" placeholder="Enter a location" id="{id}-query" />

        <details class="instructions">
            <summary class="pointer">How to use</summary>
            <ol>
                <li>
                    Either click on <code>Enter a stop number</code> to enter a stop number, or enter a location in the
                    search box.<br />
                    A location can be one of three types:
                    <ul>
                        <li>An address (e.g. <code>1000 Main Street</code>)</li>
                        <li>A point of interest (e.g. <code>Canada Life Centre</code>), or</li>
                        <li>An intersection (e.g. <code>Portage @ Main</code></li>
                    </ul>
                </li>
                <li>Speed up and send the query instantly, press <code>Enter</code></li>
                <li>Click on the search result below to select it</li>
            </ol>
        </details>

        <hr />

        {#each searchResults as result, index}
            {#if result.type === "address"}
                <input
                    type="button"
                    value="{result['street-number']} {result.street.name}"
                    on:click={() => submit(index)}
                    tabindex={index}
                />
            {:else if result.type === "intersection"}
                <input
                    type="button"
                    value="{result.street.name} @ {result['cross-street'].name}"
                    on:click={() => submit(index)}
                    tabindex={index}
                />
            {:else if result.type === "monument"}
                <input type="button" value={result.name} on:click={() => submit(index)} />
            {:else if result.type === "point"}
                <input
                    type="button"
                    value="({result.centre.latitude}, {result.centre.longitude})"
                    on:click={() => submit(index)}
                    tabindex={index}
                />
            {:else}
                <div class="error">Unknown API Location type! Please see the console for more details</div>
            {/if}
        {/each}
    </div>
</dialog>
<input class="location-search-btn" type="button" on:click={openDialog} value={placeholder} />

<style lang="sass">
    @use "../../styles/variables.sass" as var

    hr
        width: 100%

    dialog::backdrop
        background: black
        opacity: 75%

    div.contents
        display: flex
        flex-direction: column
        justify-content: center
        gap: 0.5rem
        width: 100%

    .or-label
        display: flex
        justify-content: center
        padding: 0
        margin: 0

    .instructions
        font-size: var.$font-size
</style>
