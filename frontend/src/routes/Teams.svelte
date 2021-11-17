<script lang="ts">
  import type { AxiosError } from "axios";
  import { Team, getTeams, Resource } from "../api/teams";
  import {
    AllocationMap,
    getAllocations,
    NewResourceAllocation,
    NewResourceAllocationPretty,
    putAllocations,
    ResourceAllocation,
  } from "../api/allocations";
  import {
    useQuery,
    Query,
    useQueries,
    UseQueryOptions,
    useMutation,
    useQueryClient,
  } from "@sveltestack/svelte-query";
  import AllocationModal from "../components/AllocationModal.svelte";
  import ResourceModal from "../components/ResourceModal.svelte";
  import { dateToYMD, idMap } from "../util";
  import { getProjects, Project } from "../api/projects";
  import { Component, getComponents } from "../api/components";
  import dayjs from "dayjs";
  import { timeUntilStale } from "@sveltestack/svelte-query/dist/queryCore/core/utils";

  const teamsResult = useQuery<Team[], AxiosError>("teams", getTeams);
  const projectsResult = useQuery<Project[], AxiosError>("projects", getProjects);
  const componentsResult = useQuery<Component[], AxiosError>("components", getComponents);

  let projectMap = {};
  let componentMap = {};
  $: projectMap = idMap($projectsResult.data || []);
  $: componentMap = idMap($componentsResult.data || []);

  const current_week = dayjs().startOf("week").add(1, "day");
  let start_week = current_week.add(-1, "week");
  let days = [
    start_week.toDate(),
    start_week.add(1, "week").toDate(),
    start_week.add(2, "week").toDate(),
  ];

  const fetchAllocations = async (date: Date) => {
    return await getAllocations(date);
  };

  function queryOptions(days): UseQueryOptions<AllocationMap, AxiosError>[] {
    return days.map((day) => {
      return {
        queryKey: ["alloc", dateToYMD(day)],
        queryFn: () => fetchAllocations(day),
      };
    });
  }

  $: {
    days = [
      start_week.toDate(),
      start_week.add(1, "week").toDate(),
      start_week.add(2, "week").toDate(),
    ];
  }

  let allocResult = useQueries(queryOptions(days));
  $: allocResult = useQueries(queryOptions(days));

  function advance(weeks: number) {
    start_week = start_week.add(weeks, "week");
  }

  function setCurrentWeek() {
    dayjs().startOf("week").add(1, "day");
  }

  function isCurrentWeek(date: Date): boolean {
    return current_week.isSame(date, "day");
  }

  function resourceAlloc(data: any, date: Date, resourceId: number): ResourceAllocation[] {
    const ymd = dateToYMD(date);
    const res = String(resourceId);
    return data?.[ymd]?.[res] || [];
  }

  type SrcDest = {
    src: { allocations: ResourceAllocation[]; date: Date; resourceId: number };
    dest: Date;
  };
  function srcDest(data: any, date: Date, resourceId: number, destDate): SrcDest {
    return {
      src: { allocations: resourceAlloc(data, date, resourceId), date, resourceId },
      dest: destDate,
    };
  }
  const queryClient = useQueryClient();
  const copyAllocations = useMutation<void, AxiosError, SrcDest>(
    ({ src, dest }) => {
      const newAllocations: NewResourceAllocation[] = src.allocations.map((a) => {
        let clone: ResourceAllocation = { ...a };
        delete clone.id;
        return clone;
      });
      return putAllocations(src.resourceId, dest, newAllocations);
    },
    {
      onSuccess: (resData, { src, dest }) => {
        queryClient.invalidateQueries(["alloc", dateToYMD(dest)]);
      },
    }
  );

  let alloc_modal_active = false;
  let alloc_modal_resource: Resource;
  let alloc_modal_date: Date;
  let alloc_modal_data: NewResourceAllocationPretty[];

  function editAllocations(col: number, resource: Resource) {
    alloc_modal_resource = resource;
    alloc_modal_date = days[col];

    let data = resourceAlloc($allocResult[col].data, days[col], resource.id);
    alloc_modal_data = data.map((a) => ({
      project: projectMap[a.project_id].name,
      component: componentMap[a.component_id].name,
      percent: a.percent,
    }));
    alloc_modal_active = true;
  }

  let res_modal_active = false;
  let res_modal_team_id;

  function newResource(team_id: number) {
    res_modal_team_id = team_id;
    res_modal_active = true;
  }
</script>

<div>
  <AllocationModal
    bind:is_active={alloc_modal_active}
    allocations={alloc_modal_data}
    date={alloc_modal_date}
    resource={alloc_modal_resource}
  />
  <ResourceModal bind:is_active={res_modal_active} team_id={res_modal_team_id} />

  <div class="column is-three-quarters is-offset-one-quarter">
    <nav class="pagination is-small" role="navigation" aria-label="pagination">
      <button on:click={() => advance(-1)} class="pagination-previous">Previous</button>
      <button on:click={() => advance(1)} class="pagination-next">Next</button>
    </nav>
  </div>

  {#if $teamsResult.status === "loading"}
    <span>Loading...</span>
  {:else if $teamsResult.status === "error"}
    <span>Error: {$teamsResult.error.message}</span>
  {:else}
    <div>
      {#each $teamsResult.data as team}
        <h3 class="subtitle">{team.name}</h3>
        <table class="table is-hoverable is-striped is-bordered is-fullwidth">
          <thead>
            <tr>
              <th>Resource</th>
              <th style="width: 25%;" class:is-selected={isCurrentWeek(days[0])}
                >{dayjs(days[0]).format("ll")}</th
              >
              <th style="width: 25%;" class:is-selected={isCurrentWeek(days[1])}
                >{dayjs(days[1]).format("ll")}</th
              >
              <th style="width: 25%;" class:is-selected={isCurrentWeek(days[2])}
                >{dayjs(days[2]).format("ll")}</th
              >
            </tr>
          </thead>
          <tbody>
            {#each team.resources as resource}
              <tr>
                <td>{resource.name}</td>
                {#each days as day, i}
                  <td
                    class="is-clickable"
                    class:is-selected={isCurrentWeek(day)}
                    on:click={() => editAllocations(i, resource)}
                  >
                    {#if $allocResult[i].isSuccess}
                      {#each resourceAlloc($allocResult[i].data, day, resource.id) as alloc}
                        <div>
                          {projectMap[alloc.project_id].name}
                          <span class="tag">{alloc.percent}%</span>
                        </div>
                      {:else}
                        {#if i > 0 && resourceAlloc($allocResult[i - 1].data, days[i - 1], resource.id).length > 0}
                          <button
                            class="button is-small"
                            style="left: -28px;"
                            on:click|preventDefault|stopPropagation={() =>
                              $copyAllocations.mutate(
                                srcDest($allocResult[i - 1].data, days[i - 1], resource.id, days[i])
                              )}
                          >
                            <span class="icon"><i class="fas fa-angle-double-right" /></span>
                          </button>
                        {/if}
                      {/each}
                    {/if}
                  </td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
        <div class="field pb-6">
          <div class="control">
            <button class="button is-primary is-small" on:click={() => newResource(team.id)}>
              <span class="fas fa-plus" />
              &nbsp; New Resource
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="column is-three-quarters is-offset-one-quarter">
    <nav class="pagination is-small" role="navigation" aria-label="pagination">
      <button on:click={() => advance(-1)} class="pagination-previous">Previous</button>
      <button on:click={() => advance(1)} class="pagination-next">Next</button>
    </nav>
  </div>
</div>

<style>
  .table td {
    vertical-align: middle;
  }
</style>
