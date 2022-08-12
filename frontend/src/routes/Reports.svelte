<script lang="ts">
    import type { AxiosError } from "axios";
    import { getProjects, type Project } from "../api/projects";
    import ProjectAllocationChart from "../components/charts/ProjectAllocationChart.svelte";
    import { useQueries, useQuery, type UseQueryOptions } from "@sveltestack/svelte-query";
    import { currentWeek, fmtDate, isCurrentWeek, waitForElement } from "../util";
    import dayjs from "dayjs";
    // import { Allocation, AllocationMap, getAllocations } from "../api/allocations";

    import { onMount, tick } from "svelte";
    // import { getProjectAggregates, type ProjectAggregate } from "../api/aggregates";

    let start_week = currentWeek.add(-1, "week");
    let days = [];
    // $: days = Array.from(Array(5).keys(), (i) => start_week.add(i, "week").toDate());

    // const projectsResult = useQuery<Project[], AxiosError>("projects", getProjects);

    // Takes data arg so that calling site is reactive
    // function projectAgg(data: ProjectAggregate[], date: Date, projectId: number): ProjectAggregate {
    //     const ymd = fmtDate(date);
    //     let d = dayjs(date)
    //     return data?.find((a) => a.project_id === projectId && d.isSame(a.start_date, "day"));
    // }

    // function queryOptions(days): UseQueryOptions<ProjectAggregate[], AxiosError>[] {
    //     return days.map((day) => {
    //         return {
    //             queryKey: ["projMonthAgg", fmtDate(day)],
    //             queryFn: () => getProjectAggregates(day, day.endOf('month')),
    //         };
    //     });
    // }

    // let aggResult;
    // $: aggResult = useQueries(queryOptions(days));





    onMount(async () => {
        // await waitForElement(".datatable table");
        // document
        //     .querySelector(".datatable table")
        //     .classList.add("table", "is-bordered", "is-striped", "is-hoverable");
    });


</script>

<div>
    <ProjectAllocationChart />
</div>

<style>

</style>
