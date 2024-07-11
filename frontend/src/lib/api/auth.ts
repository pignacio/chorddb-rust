import { apiCall, type FetchApi, type FetchResult } from '$lib/api';
import * as v from 'valibot';

export const UserDataSchema = v.object({
	loggedIn: v.boolean(),
	email: v.optional(v.string())
});

export type UserData = v.InferOutput<typeof UserDataSchema>;

export const ANONYMOUS_USER: UserData = {
	loggedIn: false,
	email: undefined
};

export async function getUserData(fetch: FetchApi): Promise<FetchResult<UserData>> {
	return apiCall(fetch, '/api/auth/user', UserDataSchema);
}

export async function login(
	fetch: FetchApi,
	user: string,
	password: string
): Promise<FetchResult<UserData>> {
	return apiCall(fetch, '/api/auth/login', UserDataSchema, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			user,
			password
		})
	});
}
