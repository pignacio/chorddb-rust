import type { LayoutServerLoad } from './$types';
import { unpackOrRedirect } from '$lib/api';
import { getUserData, type UserData } from '$lib/api/auth';

export const load: LayoutServerLoad = async ({ fetch }) => {
	const currentUser: UserData = unpackOrRedirect(await getUserData(fetch));
	return {
		currentUser: currentUser
	};
};
