<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { info, error } from "tauri-plugin-log-api";

    function filter() {
        function getValueOrDefault(elementId: string, def: any) {
            let element = document.getElementById(elementId) as HTMLInputElement;
            if (element) {
                return element.value;
            } else {
                return def;
            }
        }

        let category: string = getValueOrDefault("category", "Transit");
        let priority: number = parseInt(getValueOrDefault("priority", 3));
        let maxAge: number = parseInt(getValueOrDefault("maxAge", NaN));
        let limit: number = parseInt(getValueOrDefault("limit", NaN));

        // Double logging to also log to stdout
        console.log(`[ServiceAdvisory] Reloading with filters: ${category}, ${priority}, ${maxAge}, ${limit}`);
        info(`[ServiceAdvisory] Reloading with filters: ${category}, ${priority}, ${maxAge}, ${limit}`);

        let filters: any[] = [
            { Category: category },
            { Priority: priority }
        ];

        if (isFinite(limit)) filters.push({ Limit: limit });
        if (isFinite(maxAge)) filters.push({ MaxAge: maxAge });

        invoke("service_advisorie_html", { filters: filters, header: 4 })
            .then((res: string) => {
                let container = document.getElementById("advisory-container")!;
                console.log("[ServiceAdvisory] Got current advisories", res);
                info("[ServiceAdvisory] Got current advisories", res)
                container.innerHTML = res;
                return res;
            }).catch((err) => {
                console.error(err);
                error(err);
                return err;
            });
    }

    filter()
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
        <input type="number" name="MaxAge" id="maxAge">

        <label for="limit">Limit of Advisories</label>
        <input type="number" name="Limit" id="limit">

        <input type="button" id="reload" on:click={filter} value="Filter">
    </form>

    <hr/>

    <div id="advisory-container">
        <div id="loading">
            Loading Service Advisories...
        </div>
    </div>
</div>

<style lang="sass">
    // TODO add styling for the filters

    // Styling the loading container
    #loading
      display: flex
      justify-content: center
      align-items: center
      height: 100%
      color: grey
</style>
