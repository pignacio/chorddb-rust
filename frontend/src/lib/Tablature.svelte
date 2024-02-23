<script lang="ts">
  import type { Tablature } from "$lib/tablature";
  import { expandAll, getBitStyle } from "$lib/tablature";

  export let tablature: Tablature;
  export let fingerings: {[key:string]: string};

  let expanded_tab : Tablature;
  $: expanded_tab = expandAll(tablature, fingerings);

</script>

<div class="whitespace-pre font-mono">
  {#each expanded_tab.lines as line}
    {#each line as bit}
      <span class={getBitStyle(bit)}>{ bit.text }</span>
    {/each}
    <br/>
  {/each}
</div>

<style>
  .chord {
    color:red;
  }
  .fingering {
    color:blue;
  }
</style>

