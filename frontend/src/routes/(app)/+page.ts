import { unpackOrThrow } from '$lib/api';
import { loadSongs } from '$lib/api/song';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	const songs = unpackOrThrow(await loadSongs(fetch));
	return {
		songs: songs
	};
};
