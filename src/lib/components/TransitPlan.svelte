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
    import Stopwatch from "svelte-bootstrap-icons/lib/Stopwatch.svelte";
    import BusFront from "svelte-bootstrap-icons/lib/BusFront.svelte";
    import ArrowRight from "svelte-bootstrap-icons/lib/ArrowRight.svelte";
    import Shuffle from "svelte-bootstrap-icons/lib/Shuffle.svelte";
    import walking from "../components/walking.svg";
    import type { Plan } from "../../types/trip_planner";
    import { onMount } from "svelte";
    import { SegmentType } from "../../types/trip_planner";
    import TripLocation from "./TripLocation.svelte";

    export let plan: Plan;
    export let id: string;

    console.log("[TransitPlan] Displaying plan: ", plan);

    let dialog: HTMLDialogElement;

    onMount(() => {
        dialog = document.getElementById(id) as HTMLDialogElement;
    });
</script>

<div
    class="plan"
    on:click={() => dialog.showModal()}
    on:keypress={(event) => {
        if (event.key === "Enter") dialog.showModal();
    }}
>
    <div class="plan-times">
        {new Date(plan.times.start).toLocaleDateString()} @
        {new Date(plan.times.start).toLocaleTimeString()}
        <ArrowRight />
        {new Date(plan.times.end).toLocaleTimeString()}
    </div>
    <div class="plan-durations">
        <img src={walking} alt="Walking Person Icon" class="walking-icon" />
        {plan.times.durations.walking} min +
        <BusFront />
        {plan.times.durations.riding} min +
        <Stopwatch />
        {plan.times.durations.waiting} min =
        {plan.times.durations.total} min
    </div>
</div>
<dialog {id}>
    <ul class="segments">
        <!-- eslint-disable @typescript-eslint/no-unsafe-member-access -->
        {#each plan.segments as segment}
            {#if segment.type === SegmentType.Ride}
                <li class="segment-ride">
                    <BusFront />
                    ({segment.times.durations.riding} min; scheduled: {new Date(
                        segment.times.start,
                    ).toLocaleTimeString()}) on
                    <span class={segment.route["badge-style"]["class-names"]["class-name"].join(" ")}>
                        Route {segment.route["badge-label"]}
                        {segment.variant.name}
                    </span>
                </li>
            {:else if segment.type === SegmentType.Walk}
                {#if plan.segments.length <= 1}
                    <li class="segment-walk">
                        <img src={walking} alt="Walking Person Icon" class="walking-icon" />
                        Walk for {plan.segments[0].times.durations.walking} min to your destination.
                    </li>
                {:else}
                    <li class="segment-walk">
                        <img src={walking} alt="Walking Person Icon" class="walking-icon" />
                        ({segment.times.durations.walking} min)
                        <ArrowRight />
                        {#if segment.from !== undefined}
                            <TripLocation location={segment.from} />
                            <ArrowRight />
                        {/if}
                        <TripLocation location={segment.to} />
                    </li>
                {/if}
            {:else if segment.type === SegmentType.Transfer}
                <li class="segment-transfer">
                    <Shuffle /> at <TripLocation location={segment.from} />
                    {#if segment.from.stop.key !== segment.to.stop.key}
                        <ArrowRight /> <TripLocation location={segment.to} />
                    {/if}
                </li>
            {:else}
                <li class="segment-unknown">
                    Unknown segment type of your plan: {segment.type}. Expected one of: <code>Ride</code>,
                    <code>Walk</code>, or <code>Transfer</code>
                </li>
            {/if}
        {/each}
    </ul>

    <input class="close-btn" type="button" value="Close" on:click={() => dialog.close()} />
</dialog>

<style lang="sass">
    @use "src/styles/variables.sass" as var

    dialog
      background-color: var(--bg)
      color: var(--fg)

      input[type=button].close-btn
          margin-top: 20px
          width: 100%

    div.plan
        padding: 0.5em
        cursor: pointer
        border-radius: 5px
        background-color: var(--wpg-blue-light)
        border: var(--wpg-blue) solid 1px
        color: var(--fg)
        &:hover
            background-color: var(--wpg-blue)

    img.walking-icon
        height: 0.8 * var.$line-height
        width: auto

    ul.segments
        list-style: none
        padding: 0
        margin: 0
        display: flex
        flex-direction: column
        gap: 20px

        //li.segment-transfer

        //li.segment-ride

        //li.segment-walk
</style>
