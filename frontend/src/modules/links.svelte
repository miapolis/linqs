<script>
  import LinkCard from "../components/link-card.svelte";
  import { api } from "../util/path";
  import { utcTimestamp } from "../util/time";
  import { onMount } from "svelte";

  let links = [];

  onMount(async () => {
    const json = await (
      await fetch(`${api()}/links`, {
        credentials: "include",
      })
    ).json();

    let properLinks = [];
    for (const link of json.links) {
      let properLink = link;

      properLink.expired = link.expires_at
        ? Date.parse(link.expires_at) < utcTimestamp()
        : false;

      properLinks.push(properLink);
    }

    links = properLinks;
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
        expired={link.expired}
      />
    {/each}
  </div>
</div>
