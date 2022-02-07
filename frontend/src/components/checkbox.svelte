<script>
  export let id;
  export let onCheck;
</script>

<input
  {id}
  class="h-4 w-4 border border-gray-300 rounded-md checked:bg-blue-600 checked:border-blue-600
          bg-transparent cursor-pointer outline-none focus:outline-2 focus-visible:outline-blue-500 transition-all"
  type="checkbox"
  on:change={(e) => onCheck(e.target.checked)}
/>
