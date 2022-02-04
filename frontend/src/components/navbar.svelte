<script>
  export let loggedIn = undefined;
  export let username = "";
  export let shouldFetch = true;

  import { onMount } from "svelte";

  const logout = async () => {
    await fetch(`${import.meta.env.PUBLIC_API}/users/logout`, {
      method: "POST",
      credentials: "include",
    });
    window.location.pathname = "/";
  };

  onMount(async () => {
    if (shouldFetch) {
      const json = await (
        await fetch(`${import.meta.env.PUBLIC_API}/users/me`, {
          credentials: "include",
        })
      ).json();
      if (json.error) {
        loggedIn = false;
      } else {
        username = json.username;
        loggedIn = true;
      }
    }
  });
</script>

<div class="w-full h-12 flex items-center px-4 text-white flex-row-reverse">
  {#if loggedIn != null}
    {#if loggedIn}
      <div
        on:click={logout}
        class="cursor-pointer text-gray-400 hover:text-white"
      >
        Sign out
      </div>
      <div class="mx-1 text-slate-300">|</div>
      <div>{username}</div>
    {:else}
      <div>Not signed in</div>
    {/if}
  {/if}
</div>
