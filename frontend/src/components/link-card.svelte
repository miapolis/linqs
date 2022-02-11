<script>
  export let path;
  export let url;
  export let uses;
  export let track_id;
  export let expired;
</script>

<div
  on:click={() => {
    const url = `${window.location.origin}/link?id=${track_id}`;
    window.location = url;
  }}
  class={`${
    !expired ? "bg-gray-900" : "bg-red-900"
  } cursor-pointer rounded-lg outline-hidden outline-offset-0
  hover:outline-slate-800 hover:outline hover:outline-2 p-3`}
>
  <div class="w-full overflow-hidden">
    <div class="flex flex-row items-center">
      <div
        class="font-bold text-md max-w-[80%] overflow-hidden overflow-ellipsis"
      >
        {path}
      </div>
      <div class="ml-2 text-gray-200 text-sm">{uses}</div>
    </div>
    <div class="text-sm text-gray-400 overflow-hidden overflow-ellipsis">
      {url}
    </div>
  </div>
</div>
