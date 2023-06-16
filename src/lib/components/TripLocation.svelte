<script lang="ts">
    import type { Location } from "../../types/trip_planner";

    export let location: Location;

    console.debug("[TripLocation] Displaying location", location);
</script>

<!-- eslint-disable @typescript-eslint/no-unsafe-member-access -->
<span>
    {#if location.address !== undefined}
        {location.address["street-number"]} {location.address["street-name"]}
    {:else if location.monument !== undefined}
        {location.monument.name}
        ({location.monument.address["street-number"]}
        {location.monument.address.street.name})
    {:else if location.intersection !== undefined}
        {location.intersection.street.name} @ {location.intersection["cross-street"].name}
    {:else if location.point !== undefined}
        ({location.point.latitude}, {location.point.longitude})
    {:else if location.stop !== undefined}
        Stop #{location.stop.key} "<span class="stop-name">{location.stop.name}</span>"
    {:else if location.destination !== undefined}
        <svelte:self location={location.destination} />
    {:else if location.origin !== undefined}
        <svelte:self location={location.origin} />
    {:else}
        Unknown Location type
    {/if}
</span>

<style lang="sass">
    .stop-name
        color: var(--wpg-blue)
</style>
