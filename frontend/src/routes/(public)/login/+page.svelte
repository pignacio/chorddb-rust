<script lang="ts">
	import type { PageData } from './$types';
	import { superForm } from 'sveltekit-superforms';

	export let data: PageData;

	const { form, enhance, constraints, message } = superForm(data.form);
</script>

<h1>Login</h1>

<form method="POST" use:enhance>
	<div><label for="user">Username</label></div>
	<div>
		<input
			class="input input-bordered"
			type="text"
			name="user"
			bind:value={$form.user}
			{...$constraints.user}
		/>
	</div>
	<div><label for="password">Password</label></div>
	<div>
		<input
			class="input input-bordered"
			type="password"
			name="password"
			bind:value={$form.password}
			{...$constraints.password}
		/>
	</div>

	<div><button class="btn btn-primary mt-10" type="submit">Login</button></div>
	{#if $message}
		<div class="alert alert-error mt-10">{$message}</div>
	{/if}
</form>
