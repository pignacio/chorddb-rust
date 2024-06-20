<script lang="ts">
	import type { FingeringChange } from '$lib/fingering_selector';
	import ChordDrawing from '$lib/ChordDrawing.svelte';
	import type { Fingering } from './api/fingerings';

	export let fingerings: Fingering[] = [];
	export let current: Fingering = fingerings.length > 0 ? fingerings[0] : { frets: [], joined: '' };
	export let chord: string;
	export let onChange: (ev: FingeringChange) => void = (_) => {};

	if (fingerings.find((fingering) => fingering.joined === current.joined) === undefined) {
		fingerings = [current, ...fingerings];
	}
	let currentIndex: number = fingerings.findIndex(
		(fingering) => fingering.joined === current.joined
	);

	function updateFingering(diff: number) {
		let newValue = (currentIndex + diff) % fingerings.length;
		if (newValue < 0) {
			newValue += fingerings.length;
		}

		currentIndex = newValue;

		let previous = current;
		current = fingerings[currentIndex];
		onChange({ previous: previous, current: current });
	}
</script>

<div class="my-4">Current chord: {chord} {current.frets}</div>

<ChordDrawing frets={current.frets} />

<div id="fingering-selector" class="join flex w-full">
	<button
		class="btn join-item flex-none"
		on:click={() => {
			updateFingering(-1);
		}}
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="1.5"
			stroke="currentColor"
			class="w-6 h-6"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="m11.25 9-3 3m0 0 3 3m-3-3h7.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
			/>
		</svg>
	</button>
	<button class="btn join-item flex-1">
		<span>
			{current.joined}
			<br />
			{currentIndex + 1} / {fingerings.length}
		</span>
		<span class="selector-spinner hidden"
			><span class="loading loading-spinner loading-sm"></span></span
		>
	</button>
	<button
		class="btn join-item flex-none"
		on:click={() => {
			updateFingering(1);
		}}
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="1.5"
			stroke="currentColor"
			class="w-6 h-6"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="m12.75 15 3-3m0 0-3-3m3 3h-7.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
			/>
		</svg>
	</button>
</div>
