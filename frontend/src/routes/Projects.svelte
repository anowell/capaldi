<script lang="ts">
    import type { AxiosError } from "axios";
    import { Project, getProjects } from "../api/projects";
    import { useQuery } from "@sveltestack/svelte-query";

    const projectsResult = useQuery<Project[], AxiosError>("projects", getProjects);
</script>

<div>
    {#if $projectsResult.status === "loading"}
        <p>Loading projects...</p>
    {:else if $projectsResult.status === "error"}
        <p>Error fetching projects: {$projectsResult.error.message}</p>
    {:else}
        <table class="table is-hoverable is-striped">
            <thead>
                <tr>
                    <td>&nbsp;</td>
                    <th>Project</th>
                    <th>Category</th>
                </tr>
            </thead>
            <tbody>
                {#each $projectsResult.data as project}
                    <tr>
                        <td><a href="#"><ion-icon name="create" /></a></td>
                        <td>{project.name}</td>
                        <td>{project.category_id}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    {/if}
</div>
