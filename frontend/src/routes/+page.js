/** @type {import('./$types').PageLoad} */
export async function load({ fetch, params }) {
  const res = await fetch("/api/songs");
  const songs = await res.json();
  return {
    songs: await songs,
  };
}
