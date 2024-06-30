<script lang="ts">
	import { page } from '$app/stores';
	import type { PageData } from './$types';

	let data: PageData;
	let url = new URL('/api/auth/login', $page.url);
	url.searchParams.append('redirect', $page.url.toString());
</script>

<svelte:head>
	<script src="https://accounts.google.com/gsi/client" async></script>
</svelte:head>

<h1>Login</h1>

<div
	id="g_id_onload"
	data-client_id={data.googleClientId}
	data-login_uri={url.toString()}
	data-auto_prompt="false"
></div>
<div
	class="g_id_signin w-64"
	data-type="standard"
	data-size="large"
	data-theme="outline"
	data-text="sign_in_with"
	data-shape="rectangular"
	data-logo_alignment="left"
></div>
