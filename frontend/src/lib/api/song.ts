import { apiCall, encodeQueryString, type FetchApi, type FetchResult } from '$lib/api';
import * as v from 'valibot';

export const SongDetailsSchema = v.object({
	author: v.string(),
	title: v.string(),
	contents: v.string()
});

export type SongDetails = v.Output<typeof SongDetailsSchema>;

export const SongSchema = v.object({
	header: v.object({
		id: v.string(),
		author: v.string(),
		title: v.string()
	}),
	contents: v.string(),
	tablature: v.array(
		v.array(
			v.object({
				type: v.string(),
				position: v.number(),
				text: v.string(),
				chord: v.nullish(v.string())
			})
		)
	),
	fingerings: v.record(v.string(), v.string()),
	original: v.string(),
	instrument: v.string()
});

export type Song = v.Output<typeof SongSchema>;

export type SongQuery = {
	instrument?: string | null;
};

export async function loadSong(
	fetch: FetchApi,
	songId: string,
	query: SongQuery
): Promise<FetchResult<Song>> {
	const url = `/api/songs/${songId}?${encodeQueryString(query)}`;
	return apiCall(fetch, url, SongSchema);
}
