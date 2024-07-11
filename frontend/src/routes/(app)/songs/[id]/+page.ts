import { unpackOrRedirect, unpackOrThrow } from '$lib/api';
import { loadSong } from '$lib/api/song';
import { fetchInstruments } from '$lib/instrument';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params, url }) => {
	const queryString = {
		instrument: url.searchParams.get('instrument')
	};
	const song = unpackOrRedirect(await loadSong(fetch, params.id, queryString));
	const instruments = unpackOrThrow(await fetchInstruments(fetch));
	return {
		id: song.header.id,
		author: song.header.author,
		title: song.header.title,
		tablature: { lines: song.tablature },
		fingerings: song.fingerings,
		original: song.original,
		instrument: song.instrument,
		instruments: instruments
	};
};
