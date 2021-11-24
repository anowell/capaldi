<script lang="ts">
    import type { AxiosError } from "axios";
    import { Project, getProjects } from "../api/projects";
    import { useQueries, useQuery, UseQueryOptions } from "@sveltestack/svelte-query";
    import { currentWeek, fmtDate, isCurrentWeek, waitForElement } from "../util";
    import dayjs from "dayjs";
    // import { Allocation, AllocationMap, getAllocations } from "../api/allocations";
    import { Datatable } from "svelte-simple-datatables";
    import Pagination from "../components/Pagination.svelte";

    import { onMount, tick } from "svelte";
import { getProjectAggregates, ProjectAggregate } from "../api/aggregates";

    let start_week = currentWeek.add(-1, "week");
    let days = [];
    $: days = Array.from(Array(5).keys(), (i) => start_week.add(i, "week").toDate());

    const projectsResult = useQuery<Project[], AxiosError>("projects", getProjects);

    // Takes data arg so that calling site is reactive
    function projectAgg(data: ProjectAggregate[], date: Date, projectId: number): ProjectAggregate {
        const ymd = fmtDate(date);
        let d = dayjs(date)
        return data?.find((a) => a.project_id === projectId && d.isSame(a.start_date, "day"));
    }

    function queryOptions(days): UseQueryOptions<ProjectAggregate[], AxiosError>[] {
        return days.map((day) => {
            return {
                queryKey: ["projAgg", fmtDate(day)],
                queryFn: () => getProjectAggregates(day),
            };
        });
    }

    let aggResult;
    $: aggResult = useQueries(queryOptions(days));

    const settings = {
        sortable: true,
        pagination: false,
        columnFilter: false,
        scrollY: false,
        // scrollX: false,
        blocks: {
            searchInput: false,
            paginationButtons: false,
            paginationRowCount: false,
        },
    };

    let rows;
    let data;
    $: {
        data = $projectsResult.data?.map((p) => {
            return days.reduce((row, date, i) => {
                row[fmtDate(date)] = projectAgg($aggResult[i].data, date, p.id)?.resources;
                return row;
            }, p);
        });
    }

    onMount(async () => {
        await waitForElement(".datatable table");
        document
            .querySelector(".datatable table")
            .classList.add("table", "is-bordered", "is-striped", "is-hoverable");
    });

    function advance(weeks: number) {
        start_week = start_week.add(weeks, "week");
    }
</script>

<div>
    <div class="column is-three-quarters is-offset-one-quarter">
        <Pagination on:previous={() => advance(-1)} on:next={() => advance(1)} />
    </div>

    {#if $projectsResult.status === "loading"}
        <p>Loading projects...</p>
    {:else if $projectsResult.status === "error"}
        <p>Error fetching projects: {$projectsResult.error.message}</p>
    {:else}
        <Datatable {settings} {data} bind:dataRows={rows}>
            <thead>
                <th data-key="name" class="is-clickable">Name</th>
                {#each days as day, i}
                    <th
                        data-key={fmtDate(day)}
                        class="is-clickable"
                        class:is-selected={isCurrentWeek(day)}
                    >
                        {dayjs(day).format("MMM D")}
                    </th>
                {/each}
            </thead>
            <tbody>
                {#if rows}
                    {#each $rows as row}
                        <tr>
                            <td>{row.name}</td>
                            {#each days as day, i}
                                <td class:is-selected={isCurrentWeek(day)}>
                                    {#if row[fmtDate(day)] > 0}
                                        {row[fmtDate(day)].toFixed(1)}
                                    {/if}
                                </td>
                            {/each}
                        </tr>
                    {/each}
                {/if}
            </tbody>
        </Datatable>
    {/if}

    <div class="column is-three-quarters is-offset-one-quarter">
        <Pagination on:previous={() => advance(-1)} on:next={() => advance(1)} />
    </div>
</div>

<style>
    :global(.datatable th) {
        border-bottom: none !important;
    }
    :global(article.dt-table) {
        border-bottom: none !important;
    }
</style>
