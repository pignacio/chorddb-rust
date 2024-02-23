/** @type {import('./$types').PageLoad} */
export async function load({ fetch, params }) {
  const res = await fetch(`/api/songs/${ params.id }`);
  const data = await res.json();
  return {
    author: data.header.author,
    title: data.header.title,
    tablature: data.tablature,
  }
}
