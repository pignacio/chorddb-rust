import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent, url}) => {
  const { currentUser } = await parent();
  if (!currentUser.loggedIn) {
    redirect(302, '/login?redirect=' + encodeURIComponent(url.toString()));
  }
};
