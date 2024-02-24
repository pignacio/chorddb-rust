<script lang="ts">
  import { slide } from 'svelte/transition';
  import Tablature from "$lib/Tablature.svelte";
  import { findFirstChord } from "$lib/tablature"
  import FingeringSelector from "$lib/FingeringSelector.svelte";
  import type { PageData } from "./$types";

  export let data: PageData;
  let currentFingerings: {[key:string]: string} = data.fingerings;

  function updateChords() {
    currentFingerings["Em7"] = "XXXXXX";
    currentFingerings = currentFingerings;
  }

  let firstChord = findFirstChord(data.tablature);
  let fingeringsEnabled: boolean = false;
  let fingerings: {[key:string]: Promise<string[]>} = {};
  let selectedChord: string | undefined = undefined;
  let tabSelectedChord: string | undefined = undefined;
  let selectedChordFingerings: Promise<string[]>;

  async function loadFingerings(chord: string): string[] {
    let fingerings = await fetch(`/api/chords/GUITAR_STANDARD/${chord}`).then(d => d.json())
    return fingerings;
  }

  $: {
    // First clear up selection status
    if (selectedChord != tabSelectedChord) {
      // Update from tablature
      selectedChord = tabSelectedChord;
      fingeringsEnabled = true;
    } else {
      // If tab selection has not changed, use checkbox as source of truth
      if (fingeringsEnabled) {
        tabSelectedChord = selectedChord = firstChord;
      } else {
        tabSelectedChord = selectedChord = undefined;
      }
    }

    if (selectedChord) {
      if (!(selectedChord in fingerings)) {
        fingerings[selectedChord] = loadFingerings(selectedChord);
      }
      selectedChordFingerings = fingerings[selectedChord]
    }
  }

</script>

<h1>{data.author} - {data.title}</h1>

<div class="flex flex-row bg-gray-100">
  <div class="flex-auto text-xl m-4 overflow-hidden">
    <Tablature tablature={data.tablature} currentFingerings={currentFingerings} bind:selectedChord={tabSelectedChord} />
  </div>
  <div class="sticky flex flex-row top-32 h-fit">
    <div>
      <label class="btn btn-circle swap swap-rotate mt-2">
        <input type="checkbox" bind:checked={fingeringsEnabled} />
        <svg class="swap-off stroke-current" xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" d="m11.25 9-3 3m0 0 3 3m-3-3h7.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
        </svg>
        <svg class="swap-on stroke-current" xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" d="m12.75 15 3-3m0 0-3-3m3 3h-7.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
        </svg>
      </label>
    </div>
    {#if fingeringsEnabled }
      <div transition:slide={{ axis:'x' }}>
        <div class="p-3 min-h-48 whitespace-nowrap w-80 bg-gray-200 rounded-l-xl">
          <h1 class="mt-0">Fingerings</h1>
          {#await selectedChordFingerings }
          Loading...
          {:then thisFingerings}
            <FingeringSelector fingerings={thisFingerings} chord={selectedChord} bind:current={currentFingerings[selectedChord]} />
            {:catch}
            Failed to load :(
          {/await}
        </div>
      </div>
    {/if}
  </div>
</div>

<button class="btn btn-outline" on:click={updateChords}>Change!</button>
