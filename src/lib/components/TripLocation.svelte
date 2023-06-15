<script lang="ts">
    import type { Location } from "../../types/trip_planner";

    export let location: Location;
</script>

<span>
    {#if location.Address !== undefined}
        {location.Address["street-number"]} {location.Address["street-name"]}
    {:else if location.Monument !== undefined}
        {location.Monument.name}
        ({location.Address["street-number"]}
        {location.Address["street-name"]})
    {:else if location.Intersection !== undefined}
        {location.Intersection.street.name} @ {location.Intersection["cross-street"].name}
    {:else if location.Point !== undefined}
        ({location.Point.latitude}, {location.Point.longitude})
    {:else}
        Unknown Location type
    {/if}
</span>
