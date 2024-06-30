import { encodeQueryString, unpackOrThrow } from '$lib/api';
import { fetchInstruments } from '$lib/instrument';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params, url }) => {
	const queryString = {
		instrument: url.searchParams.get('instrument')
	};
	const res = await fetch(`/api/songs/${params.id}?${encodeQueryString(queryString)}`);
	const data = await res.json();
	const instruments = unpackOrThrow(await fetchInstruments(fetch));
	return {
		id: data.header.id,
		author: data.header.author,
		title: data.header.title,
		tablature: { lines: data.tablature },
		fingerings: data.fingerings,
		original: data.original,
		instrument: data.instrument,
		instruments: instruments
	};
};
