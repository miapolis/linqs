<script>
  import { onMount } from "svelte";

  let result = undefined;
  let loaded = false;

  onMount(async () => {
    const urlParams = new URLSearchParams(window.location.search);
    const id = urlParams.get("id");

    const json = await (
      await fetch(`${import.meta.env.PUBLIC_API}/track?id=${id}`)
    ).json();

    if (json.status == 200) {
      result = json;
    }
    loaded = true;
  });
</script>

{#if result != undefined && loaded}
  <div class="w-4/5 mx-auto">
    <h1 class="mt-32 text-white text-3xl">
      Tracking for Link <strong>{result.link_id}</strong>
    </h1>
    <h3 class="text-gray-300 mt-2">
      Redirect URL: <strong
        ><a href={result.link_url}>{result.link_url}</a></strong
      >
    </h3>
  </div>
  <div class="w-4/5 mx-auto rounded-xl border-2 border-gray-800 mt-12 p-4">
    <table class="w-full">
      <tr>
        <th class="text-gray-300">Time</th>
        <th class="text-gray-300">IP</th>
        <th class="text-gray-300">User Agent</th>
      </tr>
      {#each result.tracks as track}
        <tr class="border-b-2 border-gray-800 p-1">
          <td class="text-gray-300 text-sm">{track.time}</td>
          <td class="text-gray-300 text-sm">{track.ip}</td>
          <td class="text-gray-300 text-sm">{track.user_agent}</td>
        </tr>
      {/each}
    </table>
  </div>
  <div class="h-10" />
{:else if loaded}
  <div class="container">
    <div class="inner">
      <h1 class="text-8xl text-gray-800 font-bold">404</h1>
      <div class="text-gray-400 size-md mt-4">
        This tracking link is invalid.
      </div>
    </div>
  </div>
{:else}
  <div />
{/if}

<style>
  .container {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .inner {
    text-align: center;
  }

  th {
    text-align: left;
  }

  td {
    padding: 10px 20px 10px 0;
  }
</style>
