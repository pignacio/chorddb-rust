<script lang="ts">
	import { goto } from '$app/navigation';
	import type { PageData } from './$types';

	export let data: PageData;
	let author: string = '';
	let title: string = '';
	let tablature: string = '';

	async function addSong() {
		console.log('About to add', { author: author, title: title, tablature: tablature });
		var data = new FormData();
		data.append('author', author);
		data.append('title', title);
		data.append('contents', tablature);

		var json_data = {
			author: author,
			title: title,
			contents: tablature
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
		<input type="text" bind:value={author} placeholder="Author" class="input input-bordered" />
		<input type="text" bind:value={title} placeholder="Title" class="input input-bordered" />
		<textarea
			placeholder="Tab"
			bind:value={tablature}
			class="textarea textarea-bordered textarea-lg col-span-2 h-64"
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
