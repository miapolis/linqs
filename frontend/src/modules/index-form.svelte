<script>
  import PrimaryButton from "../components/primary-button.svelte";
  import Input from "../components/input.svelte";

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
    <!-- <div class="whitespace-nowrap text-sm">Learn more about creating links</div> -->
    <span class="w-full" />
    <PrimaryButton content="Create Link" loading={buttonLoading} {onClick} />
  </div>
</div>
