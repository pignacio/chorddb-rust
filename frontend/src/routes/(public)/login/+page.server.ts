import type { PageServerLoad } from './$types';
import { GOOGLE_CLIENT_ID } from '$env/static/private';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ parent, url }) => {
	const { currentUser } = await parent();
	if (currentUser.loggedIn) {
		const redirectUrl = url.searchParams.get('redirect') ?? '/';
		redirect(302, redirectUrl);
	}

	return {
		googleClientId: GOOGLE_CLIENT_ID
	};
};
