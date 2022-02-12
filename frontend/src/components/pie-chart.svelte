<script>
  import "@carbon/charts/styles-g100.min.css";
  import { PieChart } from "@carbon/charts-svelte";

  export let data;
</script>

<PieChart
  {data}
  options={{
    title: "Device Distribution",
    height: "400px",
    // color: { scale: {"a": "#fff"} },
  }}
/>
