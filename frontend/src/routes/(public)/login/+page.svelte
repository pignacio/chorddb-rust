<script lang="ts">
	import { page } from '$app/stores';
	import type { PageData } from './$types';
	import GoogleSignInButton from '$lib/GoogleSignInButton.svelte';
	import { superForm } from 'sveltekit-superforms';

	export let data: PageData;

	const { form, enhance, constraints, message } = superForm(data.form);

	let googleUrl = new URL('/api/auth/login/google', $page.url);
	googleUrl.searchParams.append('redirect', $page.url.searchParams.get('redirect') ?? '/');
</script>

<svelte:head>
	<script src="https://accounts.google.com/gsi/client" async></script>
</svelte:head>

<h1>Login</h1>

{#if data.googleClientId}
	<GoogleSignInButton
		googleClientId={data.googleClientId}
		loginUri={googleUrl.toString()}
		extraClass="w-64"
	/>
{/if}

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
