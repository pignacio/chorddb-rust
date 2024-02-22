<script lang="ts">
  import FingeringSelector from "$lib/FingeringSelector.svelte";
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<h1 class="text-3xl font-bold underline">
  Hello world!
</h1>
<div class="btn"></div>
<div class="w-96">
  <FingeringSelector fingerings={["111", "222"]} onChange={(ev) => console.log(ev.previous + " => " + ev.current)}/>
</div>

<style lang="postcss">
  :global(html) {
  background-color: theme(colors.gray.100);
  }
</style>


