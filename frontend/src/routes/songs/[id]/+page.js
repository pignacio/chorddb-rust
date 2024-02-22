/** @type {import('./$types').PageLoad} */
export async function load({ fetch, params }) {
  const res = await fetch(`/api/songs/${ params.id }`);
  return await res.json();
}
