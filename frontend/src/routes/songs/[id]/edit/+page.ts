import { unpackOrRedirect } from '$lib/api';
import { loadSong, SongDetailsSchema, type SongQuery } from '$lib/api/song';
import type { PageLoad } from './$types';
import { superValidate } from 'sveltekit-superforms';
import { valibot } from 'sveltekit-superforms/adapters';

export const load: PageLoad = async ({ fetch, params, url }) => {
	const query: SongQuery = {
		instrument: url.searchParams.get('instrument')
	};
	const song = unpackOrRedirect(await loadSong(fetch, params.id, query));
	const form = await superValidate(
		{ author: song.header.author, title: song.header.title, contents: song.original },
		valibot(SongDetailsSchema)
	);
	return {
		id: song.header.id,
		form: form
	};
};
