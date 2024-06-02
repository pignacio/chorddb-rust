<script lang="ts">
	import { goto } from '$app/navigation';
	import type { PageData } from './$types';
	import { SongDetailsSchema } from '$lib/api/song';
	import { superForm } from 'sveltekit-superforms';
	import { valibot } from 'sveltekit-superforms/adapters';

	export let data: PageData;
	let submitFailed = false;
	let formElement: HTMLElement;
	let { form, enhance, constraints } = superForm(data.form, {
		SPA: true,
		validators: valibot(SongDetailsSchema),
		onUpdate: async ({ form }) => {
			if (form.valid) {
				console.log('IsValid!', form.data);
				return;
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
</script>

<h1>Editing Song&lt;{data.id}&gt;</h1>

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
	<button class="btn btn-primary text-xl mt-4 w-32" type="submit">Save</button>
</form>
