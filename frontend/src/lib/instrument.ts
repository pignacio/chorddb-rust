import * as v from 'valibot';
import { apiCall, type FetchApi, type FetchResult } from './api';

export const InstrumentSchema = v.object({
	id: v.string(),
	name: v.string(),
	description: v.string()
});

export type Instrument = v.Output<typeof InstrumentSchema>;

export async function fetchInstruments(fetch: FetchApi): Promise<FetchResult<Instrument[]>> {
	return await apiCall(fetch, '/api/instruments', v.array(InstrumentSchema));
}
