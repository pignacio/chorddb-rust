import { goto } from '$app/navigation';
import { apiCall, encodeQueryString, voidApiCall, type FetchApi, type FetchResult } from '$lib/api';
import * as v from 'valibot';

export const SongDetailsSchema = v.object({
	author: v.string(),
	title: v.string(),
	contents: v.string()
});

export type SongDetails = v.InferOutput<typeof SongDetailsSchema>;

export const SongHeaderSchema = v.object({
	id: v.string(),
	author: v.string(),
	title: v.string()
});

export type SongHeader = v.InferOutput<typeof SongHeaderSchema>;

export const SongSchema = v.object({
	header: SongHeaderSchema,
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

export type Song = v.InferOutput<typeof SongSchema>;

export type SongQuery = {
	instrument?: string | null;
};

export function loadSongs(fetch: FetchApi): Promise<FetchResult<SongHeader[]>> {
	return apiCall(fetch, '/api/songs', v.array(SongHeaderSchema));
}

export function getSongUrl(songId: string, query?: SongQuery) {
	let url = `/api/songs/${encodeURIComponent(songId)}`;
	if (query) {
		url += `?${encodeQueryString(query)}`;
	}
	return url;
}

export async function loadSong(
	fetch: FetchApi,
	songId: string,
	query: SongQuery
): Promise<FetchResult<Song>> {
	return apiCall(fetch, getSongUrl(songId, query), SongSchema);
}

export async function patchSong(
	fetch: FetchApi,
	songId: string,
	details: SongDetails
): Promise<FetchResult<void>> {
	const result = await voidApiCall(fetch, getSongUrl(songId), {
		method: 'PATCH',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(details)
	});

	if (result.success) {
		await goto(`/songs/${songId}`);
	}
	return result;
}

export async function deleteSong(fetch: FetchApi, songId: string): Promise<boolean> {
	const result = await voidApiCall(fetch, getSongUrl(songId), {
		method: 'DELETE'
	});

	return result.success;
}
