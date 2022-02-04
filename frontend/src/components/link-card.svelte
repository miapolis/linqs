<script>
  export let path;
  export let url;
  export let track_id;
</script>

<div
  on:click={() => {
    const url = `${window.location.origin}/track?id=${track_id}`;
    window.location = url;
  }}
  class="bg-gray-900 cursor-pointer rounded-lg outline-hidden outline-offset-0
  hover:outline-slate-800 hover:outline hover:outline-2 p-3"
>
  <div class="font-bold text-md">{path}</div>
  <div class="text-sm text-gray-400">{url}</div>
</div>
