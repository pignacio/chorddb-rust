<script lang="ts">
	export let googleClientId: string;
	export let loginUri: string;
	export let extraClass: string = '';
</script>

<svelte:head>
	<script src="https://accounts.google.com/gsi/client" async></script>
</svelte:head>

<div
	id="g_id_onload"
	data-client_id={googleClientId}
	data-login_uri={loginUri}
	data-auto_prompt="false"
></div>
<div
	class="g_id_signin {extraClass}"
	data-type="standard"
	data-size="large"
	data-theme="outline"
	data-text="sign_in_with"
	data-shape="rectangular"
	data-logo_alignment="left"
></div>
