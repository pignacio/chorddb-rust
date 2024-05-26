<script lang="ts">
	import { slide } from 'svelte/transition';
	import Tablature from '$lib/Tablature.svelte';
	import { findFirstChord } from '$lib/tablature';
	import FingeringSelector from '$lib/FingeringSelector.svelte';
	import type { PageData } from './$types';
	import leftArrowSvg from '$lib/assets/left-arrow.svg';
	import rightArrowSvg from '$lib/assets/right-arrow.svg';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';

	export let data: PageData;
	let currentFingerings: { [key: string]: string } = data.fingerings;

	let firstChord = findFirstChord(data.tablature);
	let fingeringsEnabled: boolean = false;
	let fingerings: { [key: string]: Promise<string[]> } = {};
	let selectedChord: string | undefined = undefined;
	let tabSelectedChord: string | undefined = undefined;
	let selectedChordFingerings: Promise<string[]>;
	let showOriginal: boolean = false;

	let selectedInstrument: string | undefined = data.instrument.id;
	$: updateInstrument(selectedInstrument);

	async function loadFingerings(chord: string): Promise<string[]> {
		let fingerings = await fetch(
			`/api/chords/${encodeURIComponent(data.instrument)}/${encodeURIComponent(chord)}`
		).then((d) => d.json());
		return fingerings;
	}

	function updateInstrument(new_instrument: string | undefined) {
		if (new_instrument && new_instrument != data.instrument) {
			$page.url.searchParams.set('instrument', new_instrument);
			goto(`?${$page.url.searchParams.toString()}`);
		}
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
			selectedChordFingerings = fingerings[selectedChord];
		}
	}

	async function toggleOriginal() {
		showOriginal = !showOriginal;
	}
</script>

<h1>{data.author} - {data.title}</h1>
{@debug selectedInstrument}
<select bind:value={selectedInstrument}>
	{#each data.instruments as instrument}
		<option value={instrument.id}>{instrument.name}</option>
	{/each}
</select>

<div class="flex flex-row bg-gray-100">
	<div class="flex-auto text-xl m-4 overflow-hidden">
		<Tablature
			tablature={data.tablature}
			{currentFingerings}
			bind:selectedChord={tabSelectedChord}
		/>
	</div>
	<div class="sticky flex flex-row top-32 h-fit">
		<div>
			<label class="btn btn-circle swap swap-rotate mt-2">
				<input type="checkbox" bind:checked={fingeringsEnabled} />
				<img
					class="swap-off stroke-current"
					src={leftArrowSvg}
					width="32"
					height="32"
					alt="Open fingerings"
				/>
				<img
					class="swap-on stroke-current"
					src={rightArrowSvg}
					width="32"
					height="32"
					alt="Close fingerings"
				/>
			</label>
		</div>
		{#if fingeringsEnabled}
			<div transition:slide={{ axis: 'x' }}>
				<div class="p-3 min-h-48 whitespace-nowrap w-80 bg-gray-200 rounded-l-xl">
					<h1 class="mt-0">Fingerings</h1>
					{#await selectedChordFingerings}
						Loading...
					{:then thisFingerings}
						{#if selectedChord != undefined}
							<FingeringSelector
								fingerings={thisFingerings}
								chord={selectedChord}
								bind:current={currentFingerings[selectedChord]}
							/>
						{/if}
					{:catch}
						Failed to load :(
					{/await}
				</div>
			</div>
		{/if}
	</div>
</div>

<button class="btn btn-primary" on:click={toggleOriginal}>
	{#if showOriginal}Hide{:else}Show{/if} original
</button>
{#if showOriginal}
	<div class="whitespace-pre font-mono">{data.original}</div>
{/if}
