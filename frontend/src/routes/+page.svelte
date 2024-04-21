<script lang="ts">
	import { goto } from '$app/navigation';
	import type { PageData } from './$types';
	import { defaults, superForm } from 'sveltekit-superforms';
	import { valibot } from 'sveltekit-superforms/adapters';
	import * as v from 'valibot';

	const SCHEMA = v.object({
		author: v.string([v.minLength(1, 'Please enter the author.')]),
		title: v.string([v.minLength(1, 'Please enter the title.')]),
		contents: v.string([v.minLength(1, 'Please enter the tablature.')])
	});

	export let data: PageData;
	let submitFailed = false;
	let { form, enhance, constraints } = superForm(defaults(valibot(SCHEMA)), {
		SPA: true,
		validators: valibot(SCHEMA),
		onUpdate: async ({ form }) => {
			console.log('onUpdate', form);
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
			}
		}
	});
</script>

<h1>Add Song</h1>

{#if submitFailed}
	<div class="alert alert-danger">The song creation failed :(</div>
{/if}
<form class="mt-4" method="POST" use:enhance>
	<div class="grid grid-cols-2 gap-4 max-w-4xl">
		<input
			type="text"
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
			class="textarea textarea-bordered textarea-lg col-span-2 h-64"
			{...$constraints.contents}
		></textarea>
	</div>
	<button class="btn btn-primary text-xl mt-4 w-32" type="submit">Add</button>
</form>

<h1>Songs</h1>

<ul class="text-xl list-inside list-image-[url($lib/assets/musical-note.svg)]">
	{#each data.songs as { id, author, title }}
		<li>
			<a href="/songs/{id}">{author} - {title}</a>
		</li>
	{/each}
</ul>
