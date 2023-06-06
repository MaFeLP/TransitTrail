<script lang="ts">
  import Greet from './lib/pages/Greet.svelte'
  import SideBar from "./lib/SideBar.svelte";

  export let mainComponent = { component: Greet, props: [] };

  function changePage(component, props) {
    mainComponent = { component, props };
  }
</script>

<div>
  <SideBar changePage={changePage} />

  <h1>Super Fancy Transit Client</h1>
  
  <main>
    <svelte:component this={mainComponent.component} {...mainComponent.props} />
  </main>
</div>

<style lang="sass">
  @import "src/styles/variables"

  main, h1
    margin-left: $nav-width + $nav-shadow-width

  h1
    display: flex
    align-items: center
    justify-content: center
</style>
