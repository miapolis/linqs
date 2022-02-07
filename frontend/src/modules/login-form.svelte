<script>
  import Input from "../components/input.svelte";
  import { api } from "./path";

  let username = "";
  let password = "";
  let usernameError = undefined;
  let passwordError = undefined;

  const onClick = async () => {
    if (!username || !password) {
      if (!username) {
        usernameError = "Username cannot be empty.";
      }
      if (!password) {
        passwordError = "Password cannot be empty.";
      }
      return;
    }

    if (password == "incorrect") {
      passwordError = "Nice try.";
      return;
    }

    const json = await (
      await fetch(`${api()}/users/login`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify({ username, password }),
      })
    ).json();

    if (json.error) {
      switch (json.error) {
        case "username_does_not_exist":
          usernameError = "Username not found.";
          break;
        case "wrong_password":
          passwordError = "The password is incorrect.";
          break;
      }
    } else {
      console.log(json);
      window.location.reload();
    }
  };
</script>

<div
  class="w-full max-w-sm border-2 border-gray-800 text-white p-5 leading-10 rounded-lg shadow-lg"
>
  <h1 class="text-xl font-bold">Sign in to Linqs</h1>
  <label class="text-gray-400 text-xs m-0 p-0 h-0" for="username">
    Username
  </label>
  <Input
    id="username"
    autofocus
    error={usernameError}
    onChange={(e) => (username = e)}
  />
  <label class="text-gray-400 text-xs m-0 p-0 h-0" for="password">
    Password
  </label>
  <Input
    id="password"
    error={passwordError}
    type="password"
    onChange={(e) => (password = e)}
  />
  <p class="text-xs text-gray-500 mt-4">
    Note: new account registrations are currently closed.
  </p>
  <div class="flex">
    <button
      on:click={onClick}
      class="flex-1 bg-blue-600 rounded-md mx-[2px] my-[2px] mt-4 shadow outline outline-2
    outline-blue-600 hover:bg-black hover:text-blue-600 focus-visible:outline-blue-500
    focus-visible:outline-4 active:bg-slate-800 transition-all">Log in</button
    >
  </div>
</div>
