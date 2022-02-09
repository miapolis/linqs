<script>
  import LinkCard from "../components/link-card.svelte";
  import { api } from "./path";
  import { onMount } from "svelte";

  let links = [];

  onMount(async () => {
    const json = await (
      await fetch(`${api()}/links`, {
        credentials: "include",
      })
    ).json();
    links = json.links;
  });
</script>

<div class="max-w-lg text-white rounded-lg shadow-lg">
  <h1 class="text-xl font-bold mb-4">Recent Links</h1>
  <div class="grid gap-3 grid-cols-2">
    {#each links as link}
      <LinkCard
        path={link.id}
        url={link.url}
        track_id={link.track_id}
        uses={link.uses}
      />
    {/each}
  </div>
</div>
