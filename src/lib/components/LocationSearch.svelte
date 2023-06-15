<script lang="ts">
    import { error, info } from "../../util";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Location } from "../../types/common";
    import { LocationType } from "../../types/common";
    import type { Settings } from "../../types/settings";
    import { onMount } from "svelte";

    //TODO do not allow to access an intersection as a destination?

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
</script>

<dialog class="search-dialog" {id} on:close={clear} on:keypress={keypress}>
    <div class="contents">
        <input type="text" placeholder="1000 Portage Ave" id="{id}-query" />

        <p>Please select your location from below</p>

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
</style>
