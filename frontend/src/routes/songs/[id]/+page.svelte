<script lang="ts">
  import { slide } from 'svelte/transition';
  import Tablature from "$lib/Tablature.svelte";
  import type { PageData } from "./$types";

  export let data: PageData;
  let fingerings: {[key:string]: string} = data.fingerings;

  function updateChords() {
    fingerings["Em7"] = "XXXXXX";
    fingerings = fingerings;
  }

  let fingeringsEnabled: boolean = false;

  // $: console.log("Fingerings enabled:", fingeringsEnabled);
</script>

<h1>{data.author} - {data.title}</h1>

<div class="flex flex-row">
  <div class="flex-auto">
    <Tablature tablature={data.tablature} {fingerings} />
  </div>
  <div class="sticky flex flex-row top-10 h-fit">
    <div>
      <label class="btn btn-circle swap swap-rotate">
        <input id="song-drawer-checkbox" type="checkbox" bind:checked={fingeringsEnabled} />
        <svg class="swap-off stroke-current" xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" d="m11.25 9-3 3m0 0 3 3m-3-3h7.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
        </svg>
        <svg class="swap-on stroke-current" xmlns="http://www.w4.org/2000/svg" width="32" height="32" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" d="m12.75 15 3-3m0 0-3-3m3 3h-7.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
        </svg>
      </label>
    </div>
    {#if fingeringsEnabled }
    <div transition:slide={{ axis:'x' }}>
      <div class="song-drawer-contents">
        <h1>Fingerings</h1>
      </div>
    </div>
    {/if}
  </div>
</div>

<button class="btn btn-outline" on:click={updateChords}>Change!</button>
