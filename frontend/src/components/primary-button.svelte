<script>
  export let content;
  export let onClick;
  export let loading = false;
</script>

<button
  class={`right-0 mx-[2px] p-[7px] text-sm ${
    !loading
      ? "bg-blue-600 outline-blue-600 hover:bg-black hover:text-blue-600 focus-visible:outline-blue-500 focus-visible:outline-4"
      : "bg-black outline-blue-600 text-blue-600 cursor-not-allowed"
  } whitespace-nowrap rounded-md outline outline-2 transition-all inline-flex items-center active:bg-slate-800`}
  tabindex="0"
  on:click={() => onClick()}
  >{#if loading}<svg
      class="animate-spin -ml-px mr-3 h-5 w-5 text-white"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <path
        fill={!loading ? "white" : "#2563eb"}
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>{/if}{content}</button
>
