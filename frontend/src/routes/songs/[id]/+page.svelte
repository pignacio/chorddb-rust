<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;

</script>

<h1>{data.author} - {data.title}</h1>
