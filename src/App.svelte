<script lang="ts">
    import Greet from "./lib/pages/Greet.svelte";
    import SideBar from "./lib/SideBar.svelte";
    import { SvelteComponent } from "svelte";

    export let mainComponent: { component: SvelteComponent; props: object[] } = { component: Greet, props: [] };

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
