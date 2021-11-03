<script lang="ts">
  import type { AxiosError } from "axios";
  import { Team, getTeams } from "../api/teams";
  import {
    getAllocations,
    NewResourceAllocationPretty,
    ResourceAllocation,
  } from "../api/allocations";
  import { useQuery, useQueries, useInfiniteQuery } from "@sveltestack/svelte-query";
  import AllocationModal from "../components/AllocationModal.svelte";
  import ResourceModal from "../components/ResourceModal.svelte";
  import { addDays, dateToYMD, pickMap } from "../util";
import { getProjects, Project } from "../api/projects";

  const teamsResult = useQuery<Team[], AxiosError>("teams", getTeams);
  const projectsResult = useQuery<Project[], AxiosError>("projects", getProjects);

  let projectNames = {};
  $: projectNames = pickMap($projectsResult.data || {}, "name");

  function lookupAlloc(data: any, date: Date, resource: number): ResourceAllocation[] {
    const ymd = dateToYMD(date);
    const res = String(resource);
    if (data && data[ymd] && Array.isArray(data[ymd][res])) {
      return data[ymd][res];
    } else {
      return [];
    }
  }

  const days = [addDays(-7), addDays(0), addDays(7)];
  const fetchAllocations = async (date: Date) => {
    let alloc = await getAllocations(date);
    // return teamBy(alloc, a => String(a.resource_id))
    return alloc;
  };
  const allocResult = useQueries([
    { queryKey: ["alloc", days[0]], queryFn: () => fetchAllocations(days[0]) },
    { queryKey: ["alloc", days[1]], queryFn: () => fetchAllocations(days[1]) },
    { queryKey: ["alloc", days[2]], queryFn: () => fetchAllocations(days[2]) },
  ]);


  let alloc_modal_active = false;
  let alloc_modal_resource: string;
  let alloc_modal_date: Date;
  let alloc_modal_data: NewResourceAllocationPretty[];

  function editAllocations(col: number, resource) {
    alloc_modal_resource = resource.name;
    alloc_modal_date = days[col];
    // TODO: stop hard-coding allocResult[0]
    let data = lookupAlloc($allocResult[col].data, days[col], resource.id);
    alloc_modal_data = data.map((a) => ({
      project: projectNames[a.project_id],
      component: "TODO Component",
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
  {#if $teamsResult.status === "loading"}
    <span>Loading...</span>
  {:else if $teamsResult.status === "error"}
    <span>Error: {$teamsResult.error.message}</span>
  {:else}
    <div>
      {#each $teamsResult.data as team}
        <h3 class="subtitle">{team.name}</h3>
        <table class="table is-hoverable is-striped is-fullwidth">
          <thead>
            <tr>
              <th>Resource</th>
              <th style="width: 25%;">{days[0].toLocaleDateString()}</th>
              <th style="width: 25%;" class="is-selected">{days[1].toLocaleDateString()}</th>
              <th style="width: 25%;">{days[2].toLocaleDateString()}</th>
            </tr>
          </thead>
          <tbody>
            {#each team.resources as resource}
              <tr>
                <td>{resource.name}</td>
                {#each [0, 1, 2] as i}
                  <td
                    class="is-clickable"
                    class:is-selected={i === 1}
                    on:click={() => editAllocations(i, resource)}
                  >
                    {#if $allocResult[i].isSuccess}
                      {#each lookupAlloc($allocResult[i].data, days[i], resource.id) as alloc}
                        <div>
                          {projectNames[alloc.project_id]}
                          <span class="tag">{alloc.percent}%</span>
                        </div>
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
</div>