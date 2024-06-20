import { apiCall, type FetchApi, type FetchResult } from '$lib/api';
import * as v from 'valibot';

export const FingeringSchema = v.object({
	frets: v.array(v.string()),
	joined: v.string()
});

export type Fingering = v.Output<typeof FingeringSchema>;

export async function loadFingerings(
	fetch: FetchApi,
	instrument: string,
	chord: string
): Promise<FetchResult<Fingering[]>> {
	return apiCall(
		fetch,
		`/api/chords/${encodeURIComponent(instrument)}/${encodeURIComponent(chord)}`,
		v.array(FingeringSchema)
	);
}
