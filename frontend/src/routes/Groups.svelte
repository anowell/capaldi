<script lang="ts">
    import type { AxiosError } from "axios";
    import { Group, getGroups } from '../api/groups';
    import { useQuery } from '@sveltestack/svelte-query';

    const groupsResult = useQuery<Group[], AxiosError>('groups', getGroups);
</script>

<div>
    <h1>Groups</h1>
    {#if $groupsResult.status === 'loading'}
        <span>Loading...</span>
    {:else if $groupsResult.status === 'error'}
        <span>Error: {$groupsResult.error.message}</span>
    {:else}
        <div>
            {#each $groupsResult.data as group}
              <p>{group.name}</p>
            {/each}
        </div>
    {/if}
</div>
