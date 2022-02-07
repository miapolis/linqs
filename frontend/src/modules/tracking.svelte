<script>
  import Navbar from "../components/navbar.svelte";
  import Clipboard from "svelte-clipboard";
  import { api, main } from "./path";
  import { onMount } from "svelte";

  let result = undefined;
  let loaded = false;
  let copiedLink = false;

  onMount(async () => {
    const urlParams = new URLSearchParams(window.location.search);
    const id = urlParams.get("id");

    const json = await (
      await fetch(`${api()}/track?id=${id}`, {
        credentials: "include",
      })
    ).json();

    if (json.status == 200) {
      result = json;
    }
    loaded = true;
  });
</script>

<Navbar shouldFetch={true} />
<div class="container max-w-5xl mx-auto px-4">
  {#if result != undefined && loaded}
    <div class="w-4/5 mx-auto">
      <div class="flex flex-row items-center mt-32">
        <h1 class="text-white text-3xl">
          Tracking for Link <strong>{result.link_id}</strong>
        </h1>
        <Clipboard
          text={`${main()}/${result.link_id}`}
          let:copy
          on:copy={() => {
            copiedLink = true;
            setTimeout(() => {
              copiedLink = false;
            }, 2000);
          }}
        >
          <img
            width="40"
            height="40"
            class="ml-2 cursor-pointer"
            src="https://img.icons8.com/glyph-neue/64/ffffff/link.png"
            alt="copy"
            on:click={copy}
          />
          {#if copiedLink}
            <span class="text-gray-200 ml-2">Link copied!</span>
          {/if}
        </Clipboard>
      </div>
      <h3 class="text-gray-300 mt-2">
        Redirect URL: <strong
          ><a href={result.link_url}>{result.link_url}</a></strong
        >
      </h3>
    </div>
    <div class="w-4/5 mx-auto rounded-xl border-2 border-gray-800 mt-12 p-4">
      {#if result.tracks.length}
        <table>
          <tr>
            {#if result.fields.includes("Time")}
              <th class="text-gray-300">Time</th>

              <th class="text-gray-300">IP</th>
            {/if}
            {#if result.fields.includes("UserAgent")}
              <th class="text-gray-300">User Agent</th>
            {/if}
          </tr>
          {#each result.tracks as track}
            <tr class="border-b-2 border-gray-800 p-1">
              {#if result.fields.includes("Time")}
                <td class="text-gray-300 text-sm">{track.time.split(".")[0]}</td
                >
              {/if}
              {#if result.fields.includes("Ip")}
                <td class="text-gray-300 text-sm">{track.ip}</td>
              {/if}
              {#if result.fields.includes("UserAgent")}
                <td class="text-gray-300 text-sm">{track.user_agent}</td>
              {/if}
            </tr>
          {/each}
        </table>
      {:else}
        <div class="text-white text-lg">No results</div>
      {/if}
    </div>
    <div class="h-10" />
  {:else if loaded}
    <div class="container-404">
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
</div>

<style>
  .container-404 {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .inner {
    text-align: center;
  }

  table {
    display: block;
    border-collapse: collapse;
    white-space: nowrap;
    border-spacing: 0;
    overflow-x: auto;
    width: 100%;
  }

  th {
    text-align: left;
  }

  td {
    padding: 10px 20px 10px 0;
  }
</style>
