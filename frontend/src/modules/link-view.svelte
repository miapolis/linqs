<script>
  import Navbar from "../components/navbar.svelte";
  import Clipboard from "svelte-clipboard";
  import PieChart from "../components/pie-chart.svelte";
  import { api, apiBase } from "../util/path";
  import parser from "ua-parser-js";
  import { formatDifference, utcTimestamp } from "../util/time";
  import { onMount } from "svelte";

  let result = undefined;
  let loaded = false;
  let copiedLink = false;
  let expired = false;
  export let id;

  let oses = new Map();
  let data = [];

  onMount(async () => {
    const urlParams = new URLSearchParams(window.location.search);
    id = urlParams.get("id");

    const json = await (
      await fetch(`${api()}/links/${id}`, {
        credentials: "include",
      })
    ).json();

    if (json.status == 200) {
      result = json;
    }

    expired =
      (result.max_uses && result.uses >= result.max_uses) ||
      (result.expires_at && Date.parse(result.expires_at) < utcTimestamp());
    loaded = true;

    for (const track of result.tracks) {
      if (track.user_agent) {
        let os = parser(track.user_agent).os.name;
        if (os) {
          if (oses.get(os)) {
            oses.set(os, oses.get(os) + 1);
          } else {
            oses.set(os, 1);
          }
        }
      }
    }

    data = Array.from(oses.entries()).map(([os, count]) => ({
      group: os,
      value: count,
    }));
  });

  const deleteLink = async () => {
    await fetch(`${api()}/links/${id}`, {
      method: "DELETE",
      credentials: "include",
    });
    window.location.pathname = "/";
  };
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
          text={`${apiBase()}/${result.link_id}`}
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
      <div
        class="text-gray-400 mt-2 hover:text-gray-200 cursor-pointer"
        on:click={() => deleteLink()}
      >
        delete
      </div>
      <div class="w-full mt-6">
        {#if expired}
          <div class="text-red-400">Expired</div>
        {/if}
        <div class="text-gray-300">
          Uses: <strong class="text-white">{result.uses}</strong>
          {#if result.max_uses}
            / {result.max_uses}
          {/if}
        </div>
        {#if result.expires_at}
          <div class="text-gray-300">
            Expire{expired ? "d" : "s"} at:
            <strong class="text-white">{result.expires_at}</strong>
          </div>
          {#if !expired}
            <div class="text-gray-300">
              Expires in: <strong class="text-white"
                >{formatDifference(
                  new Date(Date.parse(result.expires_at)),
                  new Date(utcTimestamp())
                )}</strong
              > from now
            </div>
          {/if}
        {/if}
      </div>
    </div>
    {#if data.length > 0}
      <div class="w-4/5 mx-auto mt-6">
        <PieChart {data} />
      </div>
    {/if}
    <div class="w-4/5 mx-auto rounded-xl border-2 border-gray-800 mt-12 p-4">
      {#if result.tracks.length}
        <table>
          <tr>
            {#if result.fields.includes("Time")}
              <th class="text-gray-300">Time</th>
            {/if}
            {#if result.fields.includes("Ip")}
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
