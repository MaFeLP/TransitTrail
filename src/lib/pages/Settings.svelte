<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
    import { info, error } from "tauri-plugin-log-api";

    class Settings {
        api_key: string;
        walking_distance: number;
        waiting_time: number;
        walking_speed: number;
    }

    function settingsString(settings: Settings): string {
        return `{ api_key: ${settings.api_key}, walking_distance: ${settings.walking_distance}, waiting_time: ${settings.waiting_time}, walking_speed: ${settings.walking_speed} }`
    }

    async function load() {
        info("[Settings]: Loading settings");
        let settings: Settings = await invoke("get_settings")
        info(`[Settings]: Settings loaded: ${settingsString(settings)}`);

        // Workaround: Have the password field be of type 'text' initially, but change after value has been set
        // This makes it, so that the password field has the actual token in it, which would fail if the type
        // would be 'password' from the beginning.
        let passwordElement = document.getElementById("api-key") as HTMLInputElement;
        passwordElement.value = settings.api_key;
        passwordElement.setAttribute("type", "password");

        (document.getElementById("walking-distance") as HTMLInputElement).value = settings.walking_distance.toString();
        (document.getElementById("waiting-time") as HTMLInputElement).value = settings.waiting_time.toString();
        (document.getElementById("walking-speed") as HTMLInputElement).value = settings.walking_speed.toString();
    }

    function save() {
        info("[Settings]: Updating settings");
        invoke("save_settings", {
            apiKey: (document.getElementById("api-key") as HTMLInputElement).value,
            waitingTime: (document.getElementById("waiting-time") as HTMLInputElement).value,
            walkingDistance: (document.getElementById("walking-distance") as HTMLInputElement).value,
            walkingSpeed: (document.getElementById("walking-speed") as HTMLInputElement).value,
        }).then(() => {
            info("[Settings]: Settings updated");
        }).catch((e) => {
            alert("Failed to update settings. See console for more information");
            error(`[Settings]: Failed to update settings ${e}`);
        });
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
            permissionGranted = permission === 'granted';
        }

        try {
            await invoke("test_token", {
                token: (document.getElementById("api-key") as HTMLInputElement).value,
            });

            if (permissionGranted)
                sendNotification({
                    title: "Token Test Result",
                    body: "The specified API token is valid. Press 'save' in the settings, to save your token."
                })
            else
                alert("The specified API token is valid. Press 'save' in the settings, to save your token.")
        } catch (e) {
            if (permissionGranted)
                sendNotification({
                    title: "Token Test Result",
                    body: "The specified API token is NOT valid. Please provide a valid token and try again."
                })
            else
                alert("The specified API token is NOT valid. Please provide a valid token and try again.")
            error("[Settings]: Failed to test token", e);
        }
    }

    load()
        .then(() => {
            info("[Settings]: Initial load of settings done!");
        })
        .catch(() => {
            error("[Settings]: Could not perform initial Settings load!")
        })
</script>

<div id="settings">
    <div class="setting">
        <label for="api-key">API Key</label>
        <input type="text" id="api-key"> <!--of type text for workaround-->
        <input id="btn-test" class="btn" type="button" on:click={test_token} value="Test">
    </div>
    <div class="setting">
        <label for="walking-distance">Maximum Walking distance (meters)</label>
        <input type="number" id="walking-distance" value="1000">
    </div>
    <div class="setting">
        <label for="waiting-time">Maximum wait time (minutes)</label>
        <input type="number" id="waiting-time" value="15">
    </div>
    <div class="setting">
        <label for="walking-speed">Walking Speed (km/h)</label>
        <input type="number" id="walking-speed" value="4">
    </div>
    <div class="setting">
        <input id="btn-save"  class="btn" type="button" on:click={save} value="Save">
        <input id="btn-reset" class="btn" type="button" on:click={reset} value="Reset Default">
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
</style>