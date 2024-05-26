import { encodeQueryString } from '$lib/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params, url }) => {
	const queryString = {
		instrument: url.searchParams.get('instrument')
	};
	const res = await fetch(`/api/songs/${params.id}?${encodeQueryString(queryString)}`);
	const data = await res.json();
	return {
		author: data.header.author,
		title: data.header.title,
		tablature: { lines: data.tablature },
		fingerings: data.fingerings,
		original: data.original
	};
};
