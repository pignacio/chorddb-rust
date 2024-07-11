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
	body?: unknown;
	cause?: unknown;
}

export type FetchResult<T> = FetchSuccess<T> | FetchFailure;

function fetchSuccess<T>(response: Response, payload: T): FetchSuccess<T> {
	return {
		success: true,
		response: response,
		payload: payload
	};
}

function fetchFailure(
	response: Response,
	message: string,
	body?: unknown,
	cause?: unknown
): FetchFailure {
	return {
		success: false,
		response: response,
		message: message,
		cause: cause,
		body: body
	};
}

const API_RESULT_CHECK_SCHEMA = v.object({
	success: v.boolean(),
	message: v.string()
});

function apiResultSchema<S extends v.BaseSchema>(schema: S) {
	return v.object({
		success: v.boolean(),
		message: v.string(),
		payload: v.nullish(schema)
	});
}

export function unpackOrRedirect<T>(result: FetchResult<T>): T {
	if (result.success) {
		return result.payload;
	} else if (result.response.status == 404) {
		error(404, { message: 'Not Found' });
	} else {
		throw logError(result);
	}
}

export function unpackOrThrow<T>(result: FetchResult<T>): T {
	if (result.success) {
		return result.payload;
	}
	throw logError(result);
}

function logError(result: FetchFailure): Error {
	const message = `API call failed! statuscode=${result.response.status} when requesting "${result.response.url}". Error: ${result.message}`;
	console.error(message, result);
	return new Error(message);
}

export async function redirectingApiCall<TSchema extends v.BaseSchema>(
	fetch: FetchApi,
	url: string,
	schema: TSchema
): Promise<v.InferOutput<TSchema>> {
	const result = await apiCall(fetch, url, schema);

	return unpackOrRedirect(result);
}

function matchesApiResultShape(body: unknown) {
	return v.safeParse(API_RESULT_CHECK_SCHEMA, body).success;
}

export async function voidApiCall(
	fetch: FetchApi,
	url: string,
	init?: RequestInit | undefined
): Promise<FetchResult<void>> {
	const result = await apiCall(fetch, url, v.unknown(), init);
	if (result.success) {
		return fetchSuccess(result.response, undefined);
	} else {
		return fetchFailure(result.response, result.message, result.body, result.cause);
	}
}

export async function apiCall<TSchema extends v.BaseSchema>(
	fetch: FetchApi,
	url: string,
	schema: TSchema,
	init?: RequestInit | undefined
): Promise<FetchResult<v.InferOutput<TSchema>>> {
	const response = await fetch(url, init);
	if (response.ok) {
		try {
			const body: unknown = await response.json();
			if (matchesApiResultShape(body)) {
				const parseResult = v.safeParse(apiResultSchema(schema), body);
				if (!parseResult.success) {
					return fetchResultFromParseFailure(parseResult, body, response);
				}
				const output = parseResult.output;
				if (output.success) {
					return fetchSuccess(response, output.payload);
				} else {
					return fetchFailure(response, output.message);
				}
			} else {
				const parseResult = v.safeParse(schema, body);
				if (!parseResult.success) {
					return fetchResultFromParseFailure(parseResult, body, response);
				} else {
					return fetchSuccess(response, parseResult.output);
				}
			}
		} catch (err) {
			return fetchFailure(response, `Could not parse JSON payload. Err: ${err}`);
		}
	} else {
		return fetchFailure(response, `HTTP call failed`);
	}
}

function fetchResultFromParseFailure<S extends v.BaseSchema>(
	parseResult: v.SafeParseResult<S>,
	body: unknown,
	response: Response
): FetchFailure {
	if (parseResult.success) {
		return fetchFailure(
			response,
			'Tried to construct a fetch failure from a valid parse result!',
			body
		);
	}
	const firstIssue = parseResult.issues[0];
	const firstPath = firstIssue.path?.map((p) => p.key).join('.');
	return fetchFailure(
		response,
		`Failed to validate response. Got ${parseResult.issues.length} issues. First @(${firstPath}) ${firstIssue.message}`,
		body,
		parseResult.issues
	);
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
