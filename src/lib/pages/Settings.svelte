<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

    function load() {
        console.log("[Settings]: Loading settings");
        invoke("load_settings").then((settings) => {
            console.log("[Settings]: Settings loaded", settings);
            (document.getElementById("api-key") as HTMLInputElement).value = settings.api_key;
            (document.getElementById("walking-distance") as HTMLInputElement).value = settings.walking_distance;
            (document.getElementById("waiting-time") as HTMLInputElement).value = settings.waiting_time;
        }).catch((e) => {
            alert("Failed to load settings. See console for more information");
            console.error("[Settings]: Failed to load settings", e);
        });
    }

    function save() {
        console.log("[Settings]: Updating settings");
        invoke("save_settings", {
            apiKey: (document.getElementById("api-key") as HTMLInputElement).value,
            waitingTime: (document.getElementById("waiting-time") as HTMLInputElement).value,
            walkingDistance: (document.getElementById("walking-distance") as HTMLInputElement).value,
        }).then(() => {
            console.log("[Settings]: Settings updated");
        }).catch((e) => {
            alert("Failed to update settings. See console for more information");
            console.error("[Settings]: Failed to update settings", e);
        });
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
            console.error("[Settings]: Failed to test token", e);
        }
    }

    load();
</script>

<div id="settings">
    <div class="setting">
        <label for="api-key">API Key</label>
        <input type="password" id="api-key" value="">
        <input class="pointer" type="button" on:click={test_token} value="Test">
    </div>
    <div class="setting">
        <label for="walking-distance">Maximum Walking distance (meters)</label>
        <input type="number" id="walking-distance" value="1000">
    </div>
    <div class="setting">
        <label for="waiting-time">Maximum wait time (minutes)</label>
        <input type="number" id="waiting-time" value="15">
    </div>
    <input class="pointer" type="button" on:click={save} value="Save">
</div>

<<<<<<< HEAD
<style>
    #settings {
        margin: 15px;
        display: flex;
        flex-direction: column;
        justify-content: start;
        gap: 15px;
    }
    .setting {
        display: flex;
        flex-direction: row;
        justify-content: start;
        gap: 10px;
    }
    .setting label {
        width: 15em;
    }
=======
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

      label
        width: 15em
>>>>>>> stops-schedules
</style>
