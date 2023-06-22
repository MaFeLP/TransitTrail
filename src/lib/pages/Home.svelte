<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    async function openWebpage(link: string) {
        await invoke("open_webpage", { link: link });
    }
</script>

<div id="homepage">
    <h3>Setup</h3>
    <ol>
        <li>
            Create a token for the Winnipeg Transit API and copy it to your clipboard from
            <button on:click={async () => await openWebpage("https://api.winnipegtransit.com/home/user_session/new")}>
                here
            </button>.
        </li>
        <li>
            Go to the settings page and paste your token in to the field <code>API Key</code>. Then press
            <code>Test</code> on the right of the input field. If you get a notification, that the token is valid, press
            <code>save</code> and continue. If the token is invalid, please make sure you copied a valid one!
        </li>
        <li>
            Create a token for the Google Maps API and copy it to your clipboard from
            <button
                on:click={async () => {
                    await openWebpage("https://developers.google.com/maps/documentation/javascript/get-api-key");
                }}
            >
                here
            </button>.
        </li>
        <li>
            Go to the settings page and paste your token in to the field <code>Google Maps API Key</code>. Then press
            <code>Test</code> on the right of the input field. If you get a notification, that the token is valid, press
            <code>save</code> and continue. If the token is invalid, please make sure you copied a valid one!
        </li>
    </ol>
    <hr />

    <h3>What does it do?</h3>
    <h4>Stops</h4>
    <div>
        This module lets you get the current schedule for a stop by number. This means you can see which buses are
        serving this stop at what time and where they are going.

        <h5>Filters</h5>
        <ul>
            <li><b>Stop Number:</b> The stop number used to uniquely identify the stop</li>
            <li><b>Time Range:</b> Between what times you wan to see the buses</li>
            <li><b>Limits per Route:</b> How many buses you want to see per route</li>
        </ul>
    </div>
    <h4>Trip Planner</h4>
    <div>
        Plan your trip with the NaviGo engine by Winnipeg Transit (favour transit over walking) and Google Maps
        (sometimes favours walking over transit).

        <h5>Filters</h5>
        <ul>
            <li>
                <b>Select Start/Destination:</b> Click on this button to show a search field, where you can search for intersections,
                addresses, points of interests and with a click a stop number.
            </li>
            <li><b>Depart After:</b> Change this to what the time after represents.</li>
            <li>
                <b>Time & Date:</b> Change the date and time of your trip plan. <br />
                <b>Note: This only applies to the NaviGo Plans and not Google Maps!</b>
            </li>
        </ul>
    </div>
    <h4>Advisories</h4>
    <p>
        Gets you the latest advisories issued by Winnipeg Transit that might affect your travels, so check them out,
        before you leave!
    </p>
</div>

<style lang="sass">
    @use "src/styles/variables.sass" as var

    h3, h4
      text-decoration-line: underline
      text-decoration-color: var(--fg)

    div#homepage
      font-family: serif
      line-height: var.$line-height

    ol, ul
      display: flex
      flex-direction: column
      align-items: flex-start
      justify-content: flex-start
      gap: 0.5 * var.$line-height

    code
      background-color: var(--bg-light)
</style>
