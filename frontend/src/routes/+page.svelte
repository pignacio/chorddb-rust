<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;

</script>

<h1>Songs</h1>


<ul class="text-xl list-inside list-image-[url($lib/assets/musical-note.svg)]">
  {#each data.songs as { id, author, title }}
    <li>
      <a href="/songs/{ id }">{ author } - { title }</a>
    </li>
  {/each}
</ul>
