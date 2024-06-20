<script lang="ts">
	import { expandAll, getBitStyle, type TabBit, type Tablature } from '$lib/tablature';

	export let tablature: Tablature;
	export let currentFingerings: { [key: string]: Fingering };
	export let selectedChord: string | undefined = undefined;
	let expanded_tab: Tablature;
	$: expanded_tab = expandAll(tablature, currentFingerings);

	function selectBit(bit: TabBit) {
		if (bit.type == 'chord' || bit.type == 'fingering') {
			let chord = bit.chord;
			selectedChord = chord;
		}
	}
</script>

<div class="whitespace-pre font-mono leading-snug">
	{#each expanded_tab.lines as line}
		{#each line as bit}
			{#if bit.type != 'combo'}
				<span
					class={getBitStyle(bit)}
					class:chord-selected={bit.type == 'chord' && selectedChord == bit.chord}
					class:fingering-selected={bit.type == 'fingering' && selectedChord == bit.chord}
					on:click={() => selectBit(bit)}>{bit.text}</span
				>
			{:else}
				{console.log('Found unexpected combo bit', bit)}
			{/if}
		{/each}
		<br />
	{/each}
</div>

<style>
	.chord {
		color: theme('colors.red.500');
		font-weight: bold;
	}

	.chord-selected {
		background-color: theme('colors.red.800');
		color: theme('colors.white');
	}

	.fingering {
		color: theme('colors.blue.600');
	}

	.fingering-selected {
		background-color: theme('colors.blue.900');
		color: theme('colors.white');
	}
</style>
