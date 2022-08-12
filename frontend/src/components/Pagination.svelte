
<script lang="ts">
	import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();
</script>

<nav class="pagination is-small" aria-label="pagination">
    <button on:click={() => dispatch('previous')} class="pagination-previous">Previous</button>
    <button on:click={() => dispatch('next')} class="pagination-next">Next</button>
</nav>
