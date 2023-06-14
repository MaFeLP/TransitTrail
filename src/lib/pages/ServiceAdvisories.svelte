<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { info, error } from "../../util";
    import type { ServiceAdvisoryFilter } from "../../types/filters";
    import type { Category } from "../../types/service_advisories";
    import type { Priority } from "../../types/service_advisories";

    async function filter(): Promise<string> {
        function getValueOrDefault(elementId: string, def: string | number): string {
            let element = document.getElementById(elementId) as HTMLInputElement;
            if (element) {
                return element.value;
            } else {
                return def.toString();
            }
        }

        let category: Category = getValueOrDefault("category", "Transit") as Category;
        let priority: Priority = parseInt(getValueOrDefault("priority", 3)) as Priority;
        let maxAge: number = parseInt(getValueOrDefault("maxAge", NaN));
        let limit: number = parseInt(getValueOrDefault("limit", NaN));

        info(`[ServiceAdvisory] Reloading with filters: ${category}, ${priority}, ${maxAge}, ${limit}`);

        let filters: ServiceAdvisoryFilter[] = [{ Category: category }, { Priority: priority }];

        if (isFinite(limit)) filters.push({ Limit: limit });
        if (isFinite(maxAge)) filters.push({ MaxAge: maxAge });

        try {
            let html: string = await invoke("service_advisorie_html", { filters: filters, header: 4 });
            info(`[ServiceAdvisory] Got current advisories ${html}`);
            return html;
        } catch (err) {
            error(`[ServiceAdvisory] Could not get service advisories: ${err}`, err);
            throw err;
        }
    }
</script>

<div>
    <h2>Service Advisories</h2>
    <form>
        <h3>Filters</h3>
        <label for="category">Category</label>

        <select name="Category" id="category">
            <option value="All">All</option>
            <option value="Handi-Transit">HandiTransit</option>
            <option value="Transit" selected>Transit</option>
        </select>

        <label for="priority">Priority</label>
        <select name="Priority" id="priority">
            <option value="1">Very High</option>
            <option value="2">High</option>
            <option value="3" selected>Medium</option>
            <option value="4">Low</option>
            <option value="5">Very Low</option>
        </select>

        <label for="maxAge">Maximum Age (days)</label>
        <input type="number" name="MaxAge" id="maxAge" />

        <label for="limit">Limit of Advisories</label>
        <input type="number" name="Limit" id="limit" />

        <input type="button" id="reload" on:click={filter} value="Filter" />
    </form>

    <hr />

    <div id="advisory-container">
        {#await filter()}
            <div id="loading">Loading Service Advisories...</div>
        {:then html}
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html html}
        {:catch error}
            <p style="color: red">{error.message}</p>
        {/await}
    </div>
</div>

<style lang="sass">
    // Styling the loading container
    #loading
      display: flex
      justify-content: center
      align-items: center
      height: 100%
      color: grey
</style>
