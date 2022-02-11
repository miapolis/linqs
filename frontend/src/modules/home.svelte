<script>
  import IndexForm from "./index-form.svelte";
  import Links from "./links.svelte";
  import LoginForm from "./login-form.svelte";
  import Navbar from "../components/navbar.svelte";
  import { api } from "../util/path";

  import { onMount } from "svelte";

  let loggedIn = undefined;
  let username = undefined;

  onMount(async () => {
    const json = await (
      await fetch(`${api()}/users/me`, {
        credentials: "include",
      })
    ).json();
    if (json.error) {
      loggedIn = false;
    } else {
      username = json.username;
      loggedIn = true;
    }
  });
</script>

<Navbar {loggedIn} {username} shouldFetch={false} />
<div class="container max-w-5xl mx-auto px-4">
  <div class="w-4/5 mx-auto">
    <h1 class="mt-32 text-white text-6xl font-bold">LINQS</h1>
  </div>
  <div class="w-4/5 my-10 mx-auto">
    <h3 class="text-gray-300">
      An extremely performant URL shortener.<br />
      See a <strong><a href="https://lerners.io">demo.</a></strong>
    </h3>
  </div>
  <div class="w-4/5 mx-auto">
    {#if loggedIn != null && loggedIn}
      <IndexForm />
      <div class="w-full h-10" />
      <Links />
    {:else if loggedIn == false}
      <LoginForm />
    {/if}
  </div>
</div>
