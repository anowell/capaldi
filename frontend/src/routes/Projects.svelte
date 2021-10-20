<script lang="ts">
    import type { AxiosError } from "axios";
    import { Project, getProjects } from '../api/projects';
    import { useQuery } from '@sveltestack/svelte-query';

    const projectsResult = useQuery<Project[], AxiosError>('projects', getProjects);
</script>

<div>
    <h1>Projects</h1>
    {#if $projectsResult.status === 'loading'}
        <span>Loading...</span>
    {:else if $projectsResult.status === 'error'}
        <span>Error: {$projectsResult.error.message}</span>
    {:else}
        <div>
            {#each $projectsResult.data as project}
              <p>{project.name}</p>
            {/each}
        </div>
    {/if}
</div>
