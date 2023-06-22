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
    import Home from "./lib/pages/Home.svelte";
    import SideBar from "./lib/SideBar.svelte";
    import { SvelteComponent } from "svelte";

    export let mainComponent: { component: SvelteComponent; props: object[] } = { component: Home, props: [] };

    function changePage(component: SvelteComponent, props: object[]) {
        mainComponent = { component: component, props: props };
    }
</script>

<div>
    <SideBar {changePage} />

    <h1>Super Fancy Transit Client</h1>

    <main>
        <svelte:component this={mainComponent.component} {...mainComponent.props} />
    </main>
</div>

<style lang="sass">
  @use "src/styles/variables" as var

  main, h1
    margin-left: var.$nav-width + var.$nav-shadow-width

  h1
    display: flex
    align-items: center
    justify-content: center
</style>
