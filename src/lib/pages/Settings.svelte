<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/api/notification";
    import { info, error } from "../../util";
    import { Settings } from "../../types/settings";

    async function load() {
        info("[Settings]: Loading settings");
        let settings: Settings = await invoke("get_settings");
        info(`[Settings]: Settings loaded: ${settings.toString()}`);

        // Workaround: Have the password field be of type 'text' initially, but change after value has been set
        // This makes it, so that the password field has the actual token in it, which would fail if the type
        // would be 'password' from the beginning.
        let passwordElement = document.getElementById("api-key") as HTMLInputElement;
        passwordElement.value = settings.api_key;
        passwordElement.setAttribute("type", "password");

        (document.getElementById("min-waiting-time") as HTMLInputElement).value = settings.min_waiting_time.toString();
        (document.getElementById("max-waiting-time") as HTMLInputElement).value = settings.max_waiting_time.toString();
        (document.getElementById("max-transfers") as HTMLInputElement).value = settings.max_transfers.toString();
        (document.getElementById("max-walking-time") as HTMLInputElement).value = settings.max_walking_time.toString();
        (document.getElementById("walking-speed") as HTMLInputElement).value = settings.walking_speed.toString();
        (document.getElementById("advanced-search-interval") as HTMLInputElement).value =
            settings.search_interval.toString();
    }

    async function save() {
        let newSettings = new Settings(
            (document.getElementById("api-key") as HTMLInputElement).value,
            parseInt((document.getElementById("min-waiting-time") as HTMLInputElement).value),
            parseInt((document.getElementById("max-waiting-time") as HTMLInputElement).value),
            parseInt((document.getElementById("max-transfers") as HTMLInputElement).value),
            parseInt((document.getElementById("max-walking-time") as HTMLInputElement).value),
            parseInt((document.getElementById("walking-speed") as HTMLInputElement).value),
            parseInt((document.getElementById("advanced-search-interval") as HTMLInputElement).value),
        );
        info("[Settings]: Updating settings");
        console.log(`[Settings]: New settings:`, newSettings);
        try {
            await invoke("save_settings", { newSettings: newSettings });
            info("[Settings]: Settings updated");
        } catch (e) {
            alert("Failed to update settings. See console for more information");
            error(`[Settings]: Failed to update settings ${e}`);
        }
    }

    async function reset() {
        if (confirm("Are you sure you want to reset the settings?")) {
            await invoke("reset_settings");
            await load();
        }
    }

    async function test_token() {
        let permissionGranted = await isPermissionGranted();
        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === "granted";
        }

        try {
            await invoke("test_token", {
                token: (document.getElementById("api-key") as HTMLInputElement).value,
            });

            if (permissionGranted)
                sendNotification({
                    title: "Token Test Result",
                    body: "The specified API token is valid. Press 'save' in the settings, to save your token.",
                });
            else alert("The specified API token is valid. Press 'save' in the settings, to save your token.");
        } catch (e) {
            if (permissionGranted)
                sendNotification({
                    title: "Token Test Result",
                    body: "The specified API token is NOT valid. Please provide a valid token and try again.",
                });
            else alert("The specified API token is NOT valid. Please provide a valid token and try again.");
            error(`[Settings]: Failed to test token ${e}`);
        }
    }

    load()
        .then(() => {
            console.info("[Settings]: Initial Settings load complete");
        })
        .catch((e) => {
            error(`[Settings]: Could not perform initial Settings load! ${e}`);
        });

    let settingsElements = [
        { id: "min-waiting-time", name: "min-waiting-time", description: "Min Waiting Time (minutes)", type: "number" },
        { id: "max-waiting-time", name: "max-waiting-time", description: "Max Waiting Time (minutes)", type: "number" },
        { id: "max-transfers", name: "max-transfers", description: "Max Transfers", type: "number" },
        { id: "max-walking-time", name: "max-walking-time", description: "Max Walking Time (minutes)", type: "number" },
        { id: "walking-speed", name: "walking-speed", description: "Walking Speed (km/h)", type: "number" },
    ];

    let advancedSettingsElements = [
        {
            id: "search-interval",
            name: "search-interval",
            description: "How often to refresh the search entries, when searching for a location (ms)",
            type: "number",
        },
    ];
</script>

<div id="settings">
    <div class="setting">
        <label for="api-key">API Key</label>
        <input type="text" id="api-key" />
        <!--of type text for workaround-->
        <input id="btn-test" class="btn" type="button" on:click={test_token} value="Test" />
    </div>
    {#each settingsElements as element}
        <div class="setting">
            <label for={element.id}>{element.description}</label>
            <input type={element.type} id={element.id} name={element.name} />
        </div>
    {/each}
    <hr class="w-100" />
    <details>
        <summary class="pointer">Advanced Section</summary>

        {#each advancedSettingsElements as element}
            <div class="setting">
                <label for="advanced-{element.id}">{element.description}</label>
                <input type={element.type} id="advanced-{element.id}" name={element.name} />
            </div>
        {/each}
    </details>
    <hr class="w-100" />
    <div class="setting">
        <input id="btn-save" class="btn" type="button" on:click={save} value="Save" />
        <input id="btn-reset" class="btn" type="button" on:click={reset} value="Reset Default" />
    </div>
</div>

<style lang="sass">
  div#settings
    margin: 15px
    display: flex
    flex-direction: column
    justify-content: start
    gap: 15px

    div.setting
      display: flex
      flex-direction: row
      justify-content: start
      gap: 10px

      input.btn
        cursor: pointer
        border-radius: 5px

      input#btn-save
        background-color: greenyellow
        border: green solid 1px
        transition: background-color 100ms
        &:hover
          background-color: green

      input#btn-reset
        background-color: red
        border: darkred solid 1px
        &:hover
          background-color: darkred

      input#btn-test
        background-color: var(--wpg-blue-light)
        border: var(--wpg-blue) solid 1px
        &:hover
          background-color: var(--wpg-blue)

      label
        width: 15em

  summary
    margin-bottom: 15px

  .w-100
    width: 100%
</style>
