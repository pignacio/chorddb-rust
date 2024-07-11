import { fail, redirect } from '@sveltejs/kit';
import { message, superValidate } from 'sveltekit-superforms';
import { valibot } from 'sveltekit-superforms/adapters';
import * as v from 'valibot';
import type { PageServerLoad, Actions } from './$types';
import { login } from '$lib/api/auth';

const schema = v.object({
	user: v.pipe(v.string(), v.nonEmpty()),
	password: v.pipe(v.string(), v.nonEmpty())
});

export const load: PageServerLoad = async () => {
	const form = await superValidate(valibot(schema));

	return { form };
};

export const actions = {
	default: async ({ request, fetch }) => {
		const form = await superValidate(request, valibot(schema));
		if (!form.valid) {
			return fail(400, { form });
		}

		const result = await login(fetch, form.data.user, form.data.password);

		if (result.success) {
			return redirect(303, new URL(request.url).searchParams.get('redirect') ?? '/');
		}
		message(form, 'Could not authenticate');
		return { form };
	}
} satisfies Actions;
