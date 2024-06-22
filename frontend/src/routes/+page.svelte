<script lang="ts">
	import { goto } from '$app/navigation';
	import { NewSongSchema } from '$lib/song';
	import type { PageData } from './$types';
	import { defaults, superForm } from 'sveltekit-superforms';
	import { valibot } from 'sveltekit-superforms/adapters';
	import BackspaceSvg from '$lib/svg/BackspaceSvg.svelte';
	import MusicalNote from '$lib/svg/MusicalNote.svelte';
	import { deleteSong } from '$lib/api/song';

	export let data: PageData;
	let submitFailed = false;
	let formElement: HTMLElement;
	let { form, enhance, constraints } = superForm(defaults(valibot(NewSongSchema)), {
		SPA: true,
		validators: valibot(NewSongSchema),
		onUpdate: async ({ form }) => {
			if (form.valid) {
				let response = await fetch('/api/add_song', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify(form.data)
				});

				if (response.ok) {
					let data = await response.json();
					return await goto(`/songs/${data.id}`);
				} else {
					submitFailed = true;
				}
			} else {
				let errors = form.errors.author || [];
				if (errors.length > 0) {
					let input: HTMLObjectElement | null = formElement.querySelector('[name=author]');
					input?.setCustomValidity(errors[0]);
					input?.reportValidity();
				}
			}
		}
	});

	async function doDelete(id: string, author: string, title: string) {
		if (!confirm(`Are you sure you want to delete ${author} - ${title}?`)) {
			return;
		}
		const deleted = await deleteSong(fetch, id);

		if (deleted) {
			data.songs = data.songs.filter((song) => song.id !== id);
		}
	}
</script>

<h1>Songs</h1>

<ul class="text-xl">
	{#each data.songs as { id, author, title }}
		<li class="flex">
			<MusicalNote class="size-6 mr-2" />
			<a href="/songs/{id}">{author} - {title}</a>
			<button type="button" on:click={() => doDelete(id, author, title)}>
				<BackspaceSvg class="ml-2 size-6 stroke-red-500 hover:stroke-red-300" />
			</button>
		</li>
	{/each}
</ul>

<h1>Add Song</h1>

{#if submitFailed}
	<div class="alert alert-danger">The song creation failed :(</div>
{/if}
<form class="mt-4" method="POST" use:enhance bind:this={formElement}>
	<div class="grid grid-cols-2 gap-4 max-w-4xl">
		<input
			type="text"
			name="author"
			bind:value={$form.author}
			placeholder="Author"
			class="input input-bordered"
			{...$constraints.author}
		/>
		<input
			type="text"
			bind:value={$form.title}
			placeholder="Title"
			class="input input-bordered"
			{...$constraints.title}
		/>
		<textarea
			placeholder="Tab"
			bind:value={$form.contents}
			class="textarea textarea-bordered textarea-lg col-span-2 h-64 font-mono leading-none"
			{...$constraints.contents}
		></textarea>
	</div>
	<button class="btn btn-primary text-xl mt-4 w-32" type="submit">Add</button>
</form>
