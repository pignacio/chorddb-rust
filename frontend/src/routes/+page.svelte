<script lang="ts">
	import { goto } from '$app/navigation';
	import type { PageData } from './$types';
	import SuperDebug, { superForm, superValidate } from 'sveltekit-superforms';
	import { valibot } from 'sveltekit-superforms/adapters';

	import * as v from 'valibot';

	const schema = v.object({
		author: v.string(),
		title: v.string(),
		tablature: v.string()
	});

	let preForm = superValidate(valibot(schema));
	const { form } = superForm(preForm);

	export let data: PageData;

	async function addSong() {
		console.log('About to add', form);
		var json_data = {
			author: $form.author,
			title: $form.title,
			contents: $form.tablature
		};

		let response = await fetch('/api/add_song', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(json_data)
		});

		if (response.ok) {
			let data = await response.json();
			return await goto(`/songs/${data.id}`);
		} else {
			return 'FAILED';
		}
	}
</script>

<h1>Add Song</h1>

<form class="mt-4" on:submit|preventDefault={addSong}>
	<div class="grid grid-cols-2 gap-4 max-w-4xl">
		<input
			type="text"
			bind:value={$form.author}
			placeholder="Author"
			class="input input-bordered"
		/>
		<input type="text" bind:value={$form.title} placeholder="Title" class="input input-bordered" />
		<textarea
			placeholder="Tab"
			bind:value={$form.tablature}
			class="textarea textarea-bordered textarea-lg col-span-2 h-64"
		></textarea>
	</div>
	<button class="btn btn-primary text-xl mt-4 w-32" type="submit">Add</button>
</form>
<SuperDebug data={$form} />

<h1>Songs</h1>

<ul class="text-xl list-inside list-image-[url($lib/assets/musical-note.svg)]">
	{#each data.songs as { id, author, title }}
		<li>
			<a href="/songs/{id}">{author} - {title}</a>
		</li>
	{/each}
</ul>
