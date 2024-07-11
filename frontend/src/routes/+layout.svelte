<script lang="ts">
	import '../app.css';
	import type { LayoutData } from './$types';

	export let data: LayoutData;
</script>

<div class="navbar bg-neutral text-neutral-content">
	<div class="navbar-start">
		<a href="/" class="btn btn-ghost text-xl">ChordDB</a>
	</div>
	<div class="navbar-end">
		{data.currentUser.email ?? 'Anonymous'}
	</div>
</div>

<div class="container mx-auto p-4">
	<slot />
</div>
