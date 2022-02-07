<script>
  import PrimaryButton from "../components/primary-button.svelte";
  import Input from "../components/input.svelte";
  import { api, main } from "./path.js";
  import Checkbox from "../components/checkbox.svelte";

  let path = "";
  let url = "";
  let trackTime = false;
  let trackIp = false;
  let trackUserAgent = false;

  let buttonLoading = false;

  let pathError = undefined;
  let urlError = undefined;

  const onClick = async () => {
    buttonLoading = true;

    let response = await fetch(`${api()}/create`, {
      method: "POST",
      credentials: "include",
      body: JSON.stringify({
        path: path.trim() == "" ? null : path.trim(),
        url: url,
        track_time: trackTime,
        track_ip: trackIp,
        track_user_agent: trackUserAgent,
      }),
    });
    let json = await response.json();

    if (json.status == "error") {
      switch (json.message) {
        case "url is required":
          urlError = "URL is required.";
          break;
        case "path is invalid":
          pathError =
            "Path may only contain letters, numbers, dashes and underscores.";
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
        window.location = `${main()}/track?id=${json.track_id}`;
      }, 1000);

      return;
    }

    setTimeout(() => {
      buttonLoading = false;
    }, 500);
  };
</script>

<div
  class="max-w-lg border-2 border-gray-800 text-white p-5 leading-10 rounded-lg shadow-lg"
>
  <label class="text-gray-400 text-xs m-0 p-0 h-0" for="path"
    >The Path after <strong>i.lerners.io</strong> (leave empty for random)</label
  >
  <Input
    placeholder="Path"
    autofocus
    error={pathError}
    onChange={(e) => (path = e)}
  />
  <label class="text-gray-400 text-xs m-0 p-0 h-0" for="url">Redirect URL</label
  >
  <Input
    placeholder="https://google.com"
    error={urlError}
    onChange={(e) => (url = e)}
  />
  <div class="my-4 border-[1px] border-gray-900" />
  <div class="flex flex-1 justify-start items-center">
    <div class="flex w-96 flex-col gap-[6px]">
      <div class="flex flex-row items-center">
        <Checkbox id="check-time" onCheck={(c) => (trackTime = c)} />
        <label class="text-gray-300 text-sm ml-2" for="check-time"
          >Track time</label
        >
      </div>
      <div class="flex flex-row items-center">
        <Checkbox id="check-ip" onCheck={(c) => (trackIp = c)} />
        <label class="text-gray-300 text-sm ml-2" for="check-ip"
          >Track IP address</label
        >
      </div>
      <div class="flex flex-row items-center">
        <Checkbox id="check-user-agent" onCheck={(c) => (trackUserAgent = c)} />
        <label class="text-gray-300 text-sm ml-2" for="check-user-agent"
          >Track user agent</label
        >
      </div>
    </div>
    <span class="w-full" />
    <PrimaryButton content="Create Link" loading={buttonLoading} {onClick} />
  </div>
</div>
