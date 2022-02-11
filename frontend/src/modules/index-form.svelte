<script>
  import PrimaryButton from "../components/primary-button.svelte";
  import Input from "../components/input.svelte";
  import { api, main } from "../util/path.js";
  import Checkbox from "../components/checkbox.svelte";

  let path = "";
  let url = "";
  let trackTime = false;
  let trackIp = false;
  let trackUserAgent = false;
  let expiresIn = undefined;
  let maxUses = undefined;

  let buttonLoading = false;

  let pathError = undefined;
  let urlError = undefined;
  let expiresInError = undefined;
  let maxUsesError = undefined;

  const onClick = async () => {
    if (expiresIn < 1) {
      expiresInError = "Value cannot be less than 1.";
      return;
    } else if (expiresIn > 525600) {
      expiresInError = "Expiry time cannot be set to over a year.";
      return;
    }

    if (maxUses < 1) {
      maxUsesError = "Value cannot be less than 1.";
      return;
    } else if (maxUses > 2147483647) {
      maxUsesError = "Max uses set too high.";
      return;
    }

    buttonLoading = true;

    let response = await fetch(`${api()}/links/create`, {
      method: "POST",
      credentials: "include",
      body: JSON.stringify({
        path: path.trim() == "" ? null : path.trim(),
        url: url,
        track_time: trackTime,
        track_ip: trackIp,
        track_user_agent: trackUserAgent,
        expires_in_minutes: expiresIn,
        max_uses: maxUses,
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
        window.location = `${main()}/link?id=${json.track_id}`;
      }, 1000);

      return;
    }

    setTimeout(() => {
      buttonLoading = false;
    }, 500);
  };
</script>

<div
  class="max-w-lg border-2 border-gray-800 text-white p-5 rounded-lg shadow-lg"
>
  <label class="text-gray-400 text-xs p-0 h-0" for="path"
    >The Path after <strong>l.lerners.io</strong> (leave empty for random)</label
  >
  <Input
    placeholder="Path"
    autofocus
    error={pathError}
    onChange={(e) => (path = e)}
    classes="my-2"
  />
  <label class="text-gray-400 text-xs m-0 p-0 h-0" for="url">Redirect URL</label
  >
  <Input
    placeholder="https://google.com"
    error={urlError}
    onChange={(e) => (url = e)}
    classes="my-2"
  />
  <div class="my-4 border-[1px] border-gray-900" />
  <div class="flex flex-1">
    <div class="flex w-96 flex-col justify-start gap-[6px]">
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
  </div>
  <div class="w-full flex mt-4 gap-x-2">
    <div>
      <label for="expires-in" class="text-gray-400 text-xs whitespace-nowrap"
        >Expires in (minutes from now)</label
      >
      <Input
        onChange={(e) =>
          e == "" ? (expiresIn = undefined) : (expiresIn = parseInt(e))}
        id="expires-in"
        error={expiresInError}
        type="number"
        classes="my-2"
        min="1"
        max="525600"
      />
    </div>
    <div>
      <label for="max-uses" class="text-gray-400 text-xs whitespace-nowrap"
        >Max uses</label
      >
      <Input
        onChange={(e) =>
          e == "" ? (maxUses = undefined) : (maxUses = parseInt(e))}
        id="max-uses"
        error={maxUsesError}
        type="number"
        classes="my-2"
        min="1"
        max="2147483647"
      />
    </div>
  </div>
  <div class="flex mt-6 flex-row-reverse">
    <PrimaryButton content="Create Link" loading={buttonLoading} {onClick} />
  </div>
</div>
