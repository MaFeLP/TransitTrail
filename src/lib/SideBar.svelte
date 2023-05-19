<script lang="ts">
    // Page Imports
    import Home from "./pages/Home.svelte";
    import Greet from "./pages/Greet.svelte";

    // Icon imports
    import Gear from "svelte-bootstrap-icons/lib/Gear.svelte";
    import House from "svelte-bootstrap-icons/lib/House.svelte";
    import Megaphone from "svelte-bootstrap-icons/lib/Megaphone.svelte";

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
                <House/>
                Home
            </button>

            <button on:click={() => changePage(Greet, [])}>
                <Megaphone/>
                Greeter
            </button>
        </li>
    </ul>
    <ul>
        <li>
            <button on:click={() => changePage(Greet, [])}>
                <Gear />
                Settings
            </button>
        </li>
    </ul>
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

    &:hover
      box-shadow: 0 0 10px var(--wpg-blue)
</style>
