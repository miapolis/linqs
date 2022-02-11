<script>
  export let error = undefined;
  export let id = undefined;
  export let autofocus = false;
  export let placeholder = undefined;
  export let type = undefined;
  export let min = undefined;
  export let max = undefined;
  export let classes = "";
  export let onChange;
</script>

<!-- svelte-ignore a11y-autofocus -->
<input
  {id}
  class={`w-full rounded-md outline-none border-2 p-2 text-sm ${
    error == undefined
      ? "border-gray-800 focus:border-gray-400"
      : "border-red-600"
  } transition-all bg-black ` + classes}
  {placeholder}
  {autofocus}
  {type}
  {min}
  {max}
  on:input={(e) => {
    onChange(e.target.value);
  }}
/>
{#if error}
  <div class="text-gray-400 text-sm mt-2">{error}</div>
{/if}
