import type { LayoutLoad } from './$types';
import { unpackOrRedirect } from '$lib/api';
import { getUserData, type UserData } from '$lib/api/auth';

export const load: LayoutLoad = async ({ fetch }) => {
	const currentUser: UserData = unpackOrRedirect(await getUserData(fetch));
	return {
		currentUser: currentUser
	};
};
