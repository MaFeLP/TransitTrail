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
    // Page Imports
    import Home from "./pages/Home.svelte";
    import Greet from "./pages/Greet.svelte";
    import Settings from "./pages/Settings.svelte";
    import Stops from "./pages/Stops.svelte";
    import ServiceAdvisories from "./pages/ServiceAdvisories.svelte";

    // Icon imports
    import Gear from "svelte-bootstrap-icons/lib/Gear.svelte";
    import House from "svelte-bootstrap-icons/lib/House.svelte";
    import Megaphone from "svelte-bootstrap-icons/lib/Megaphone.svelte";
    import Pin from "svelte-bootstrap-icons/lib/Pin.svelte";

    /**
     * This function is used to change the page. It takes in a component and props, and then sets the page to that component with those props.
     * @param component The component to change to
     * @param props The props to pass to the component
     */
    export let changePage: (component, props) => void;
</script>

<nav>
    <ul>
        <li>
            <button on:click={() => changePage(Home, [])}>
                <House />
                Home
            </button>
        </li>
        <li>
            <button on:click={() => changePage(Greet, [])}>
                <Megaphone />
                Greeter
            </button>
        </li>
        <li>
            <button on:click={() => changePage(Stops, [])}>
                <Pin />
                Stops
            </button>
        </li>
        <li>
            <button on:click={() => changePage(ServiceAdvisories, [])}>
                <Megaphone />
                Advisories
            </button>
        </li>
    </ul>
    <div>
        <button on:click={() => changePage(Settings, [])}>
            <Gear />
            Settings
        </button>
    </div>
</nav>

<style lang="sass">
  @import "src/styles/variables"

  // A fixed-left nav bar. It has two children: the normals pages at the top, and the settings button
  nav
    position: fixed
    top: 0
    left: 0
    bottom: 0
    width: $nav-width
    box-shadow: var(--nav-shadow)
    display: flex
    flex-direction: column
    justify-content: space-between

  // The list that holds the pages
  nav ul
    margin: 0
    padding: 0
    width: 100%
    list-style: none

  // The individual page selectors
  button
    width: $nav-width - 10px
    padding: 5px
    margin: 5px
    height: 100%
    border: 2px solid var(--wpg-blue)
    border-radius: 3px
    background: var(--wpg-blue-light)
    color: var(--fg)
    font-size: 1.5em
    font-weight: bold
    text-align: start
    cursor: pointer

  button:hover
    box-shadow: 0 0 10px var(--wpg-blue)
</style>
