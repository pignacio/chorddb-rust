import * as v from 'valibot';
import { error } from '@sveltejs/kit';

export type FetchApi = (
	input: URL | RequestInfo,
	init?: RequestInit | undefined
) => Promise<Response>;

export interface FetchSuccess<T> {
	success: true;
	response: Response;
	payload: T;
}

export interface FetchFailure {
	success: false;
	response: Response;
	message: string;
	cause?: unknown;
}

export type FetchResult<T> = FetchSuccess<T> | FetchFailure;

export function unpackOrRedirect<T>(result: FetchResult<T>): T {
	if (result.success) {
		return result.payload;
	} else if (result.response.status == 404) {
		error(404, { message: 'Not Found' });
	} else {
		const message = `API call failed! statuscode=${result.response.status} when requesting "${result.response.url}". Error: ${result.message}`;
		console.error(message, result);
		error(500, { message: message });
	}
}

export function unpackOrThrow<T>(result: FetchResult<T>): T {
	if (result.success) {
		return result.payload;
	}
	console.error('API CALL FAILED', result);
	throw result;
}

export async function redirectingApiCall<TSchema extends v.BaseSchema>(
	fetch: FetchApi,
	url: string,
	schema: TSchema
): Promise<v.Output<TSchema>> {
	const result = await apiCall(fetch, url, schema);

	return unpackOrRedirect(result);
}

export async function apiCall<TSchema extends v.BaseSchema>(
	fetch: FetchApi,
	url: string,
	schema: TSchema
): Promise<FetchResult<v.Output<TSchema>>> {
	const response = await fetch(url);
	if (response.ok) {
		try {
			const body: unknown = await response.json();
			const parseResult = await v.safeParseAsync(schema, body);
			if (parseResult.success) {
				return {
					success: true,
					response: response,
					payload: parseResult.output
				};
			}
			const firstIssue = parseResult.issues[0];
			const firstPath = firstIssue.path?.map((p) => p.key).join('.');
			return {
				success: false,
				response: response,
				message: `Failed to validate response. Got ${parseResult.issues.length} issues. First @(${firstPath}) ${firstIssue.message}`,
				cause: parseResult.issues
			} satisfies FetchFailure;
		} catch (err) {
			return {
				success: false,
				response: response,
				message: `Could not parse JSON payload. Err: ${err}`
			} satisfies FetchFailure;
		}
	} else {
		return {
			success: false,
			response: response,
			message: `HTTP call failed`
		} satisfies FetchFailure;
	}
}

export function encodeQueryString(params: { [key: string]: string | null | undefined }): string {
	return Object.entries(params)
		.flatMap(([key, value]) => {
			if (value != undefined && value != null) {
				return [[key, value]];
			} else {
				return [];
			}
		})
		.map((kv) => kv.map(encodeURIComponent).join('='))
		.join('&');
}
