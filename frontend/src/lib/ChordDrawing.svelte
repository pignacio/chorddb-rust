<script lang="ts">
	import { buildChordLines, drawChord, type DrawedChord } from '$lib/chord_drawing';
	export let frets: string[];

	let lines: string[];
	$: lines = buildChordLines(frets);
	$: console.log(lines);
	let drawedChord: DrawedChord;
	$: drawedChord = drawChord(frets);
</script>

<div class="font-mono leading-none">
	{#if drawedChord.start_fret > 0}
		&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;{drawedChord.start_fret}<br />
	{/if}
	{#each drawedChord.cordas as corda}
		{@const start = drawedChord.start_fret > 0 ? drawedChord.start_fret : 1}
		<span class="flex">
			{#if corda.type === 'mute'}
				<span>|X</span>
			{:else if corda.type === 'noted' && corda.note_fret === 0}
				<span>|<span class="text-red-500 font-bold">o</span></span>
			{:else if corda.type === 'noted'}
				<span>||</span>
			{/if}
			{#if drawedChord.start_fret > 0}
				<span>...|</span>
			{/if}
			{#each { length: drawedChord.end_fret + 1 - start } as _, i}
				<span>
					{#if corda.note_fret === i + start}
						-<span class="text-red-500 font-bold">o</span>-|
					{:else}
						---|
					{/if}
				</span>
			{/each}
		</span>
	{/each}
</div>
