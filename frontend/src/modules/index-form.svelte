<script>
  import PrimaryButton from "../components/primary-button.svelte";

  let path = "";
  let url = "";
  let buttonLoading = false;

  let pathError = undefined;
  let urlError = undefined;

  const onClick = async () => {
    buttonLoading = true;

    let response = await fetch(
      `${import.meta.env.PUBLIC_API}/create?path=${path}&url=${url}`,
      {
        method: "POST",
      }
    );
    let json = await response.json();

    if (json.status == "error") {
      switch (json.message) {
        case "url is required":
          urlError = "URL is required.";
          break;
        case "url is invalid":
          urlError = "That URL is invalid.";
          break;
        case "path already exists":
          pathError = "That path already exists.";
          break;
      }
    } else {
      pathError = undefined;
      urlError = undefined;

      setTimeout(() => {
        buttonLoading = false;
        window.location = `${import.meta.env.PUBLIC_URL}/track?id=${
          json.track_id
        }`;
      }, 1000);

      return;
    }

    setTimeout(() => {
      buttonLoading = false;
    }, 500);
  };
</script>

<div
  class="w-4/5 mx-auto border-2 border-gray-800 text-white p-5 leading-10 rounded-lg shadow-lg"
>
  <label class="text-gray-400 text-xs m-0 p-0 h-0" for="path"
    >The Path after <strong>i.lerners.io</strong> (leave empty for random)</label
  >
  <!-- svelte-ignore a11y-autofocus -->
  <input
    id="path"
    class={`w-full rounded-md outline-none border-2 p-2 text-sm ${
      pathError == undefined
        ? "border-gray-800 focus:border-gray-400"
        : "border-red-600"
    } transition-all bg-black`}
    placeholder="Path"
    autofocus
    bind:value={path}
  />
  {#if pathError}
    <div class="text-gray-400 text-sm mt-2">{pathError}</div>
  {/if}
  <label class="text-gray-400 text-xs m-0 p-0 h-0" for="url">Redirect URL</label
  >
  <input
    id="url"
    class={`w-full rounded-md outline-none border-2 p-2 text-sm ${
      urlError == undefined
        ? "border-gray-800 focus:border-gray-400"
        : "border-red-600"
    } transition-all bg-black`}
    placeholder="https://google.com"
    bind:value={url}
  />
  {#if urlError}
    <div class="text-gray-400 text-sm mt-2">{urlError}</div>
  {/if}
  <div class="my-4 border-[1px] border-gray-900" />
  <div class="flex flex-1 justify-start items-center">
    <div class="whitespace-nowrap text-sm">Learn more about creating links</div>
    <span class="w-full" />
    <PrimaryButton content="Create Link" loading={buttonLoading} {onClick} />
  </div>
</div>
