import * as v from 'valibot';

export const NewSongSchema = v.object({
	author: v.string(),
	title: v.string(),
	contents: v.string()
});
