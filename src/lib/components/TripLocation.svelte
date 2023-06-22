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
