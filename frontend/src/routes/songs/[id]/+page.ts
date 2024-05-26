import { encodeQueryString, unpackOrThrow } from '$lib/api';
import { fetchInstruments } from '$lib/instrument';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params, url }) => {
	console.log('Loading!', { params: params, url: url });
	const queryString = {
		instrument: url.searchParams.get('instrument')
	};
	const apiUrl = `/api/songs/${params.id}?${encodeQueryString(queryString)}`;
	console.log('Calling API', apiUrl);
	const res = await fetch(apiUrl);
	const data = await res.json();
	const instruments = unpackOrThrow(await fetchInstruments(fetch));
	return {
		author: data.header.author,
		title: data.header.title,
		tablature: { lines: data.tablature },
		fingerings: data.fingerings,
		original: data.original,
		instrument: data.instrument,
		instruments: instruments
	};
};
