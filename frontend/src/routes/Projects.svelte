<script lang="ts">
    import type { AxiosError } from "axios";
    import { Project, getProjects } from "../api/projects";
    import { useQueries, useQuery, UseQueryOptions } from "@sveltestack/svelte-query";
    import { currentWeek, dateToYMD, isCurrentWeek } from "../util";
    import dayjs from "dayjs";
    import { Allocation, AllocationMap, getAllocations } from "../api/allocations";

    let start_week = currentWeek.add(-1, "week");
    let days = [];
    $: days = Array.from(Array(5).keys(), (i) => start_week.add(i, "week").toDate());

    const projectsResult = useQuery<Project[], AxiosError>("projects", getProjects);

    // Takes data arg so that calling site is reactive
    function projectAlloc(data: AllocationMap, date: Date, projectId: number): Allocation[] {
        const ymd = dateToYMD(date);
        return data?.[ymd]?.filter((r) => r.project_id === projectId) || [];
    }

    function projectAllocAgg(data: AllocationMap, date: Date, projectId: number): number {
        return projectAlloc(data, date, projectId).reduce((a, b) => a + b.percent, 0) / 100;
    }


    function queryOptions(days): UseQueryOptions<AllocationMap, AxiosError>[] {
        return days.map((day) => {
            return {
                queryKey: ["allAlloc", dateToYMD(day)],
                queryFn: () => getAllocations(day, true),
            };
        });
    }

    let allocResult;
    $: allocResult = useQueries(queryOptions(days));
</script>

<div>
    {#if $projectsResult.status === "loading"}
        <p>Loading projects...</p>
    {:else if $projectsResult.status === "error"}
        <p>Error fetching projects: {$projectsResult.error.message}</p>
    {:else}
        <table class="table is-hoverable is-striped is-bordered is-full-width">
            <thead>
                <tr>
                    <th>Project</th>
                    <th>Category</th>
                    {#each days as day, i}
                        <th class:is-selected={isCurrentWeek(day)}>
                            {dayjs(day).format("MMM D")}
                        </th>
                    {/each}
                </tr>
            </thead>
            <tbody>
                {#each $projectsResult.data as project}
                    <tr>
                        <td>{project.name}</td>
                        <td>{project.category_id}</td>
                        {#each days as day, i}
                            <td class:is-selected={isCurrentWeek(day)}>
                                {#if $allocResult[i].isSuccess}
                                    {#each [projectAllocAgg($allocResult[i].data, day, project.id)] as weeks}
                                        {#if weeks > 0}
                                            {weeks.toFixed(1)}
                                        {/if}
                                    {/each}
                                {/if}
                            </td>
                        {/each}
                    </tr>
                {/each}
            </tbody>
        </table>
    {/if}
</div>
