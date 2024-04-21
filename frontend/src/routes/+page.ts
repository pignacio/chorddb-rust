import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	const res = await fetch('/api/songs');
	const songs = await res.json();
	return {
		songs: await songs
	};
};
