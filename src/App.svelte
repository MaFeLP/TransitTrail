<script lang="ts">
  import Greet from './lib/pages/Greet.svelte'
  import SideBar from "./lib/SideBar.svelte";

  export let mainComponent = { component: Greet, props: [] };

  function changePage(component, props) {
    mainComponent = { component, props };
  }
</script>

<main class="container">
  <SideBar changePage={changePage} />

  <h1>Welcome to Tauri!</h1>

  <svelte:component this={mainComponent.component} {...mainComponent.props} />
</main>

<style>
  main {
    margin-left: var(--nav-width);
  }
</style>
