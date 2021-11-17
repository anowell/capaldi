<script lang="ts">
  import type { AxiosError } from "axios";
  import { getProjects, Project } from "../api/projects";
  import { getComponents, Component } from "../api/components";
  import { useMutation, useQuery, useQueryClient } from "@sveltestack/svelte-query";
  import AutoComplete from "./AutoComplete.svelte";
  import {
deleteResourceAllocations,
    NewResourceAllocation,
    NewResourceAllocationPretty,
    putAllocations,
  } from "../api/allocations";
  import { onMount } from "svelte";
  import { dateToYMD } from "../util";
  import type { Resource } from "../api/teams";

  export let resource: Resource;
  export let date: Date;
  export let allocations: NewResourceAllocationPretty[] = [];
  export let is_active: boolean;
  let close = () => (is_active = false);

  const projectsResult = useQuery<Project[], AxiosError>("projects", getProjects);
  const componentsResult = useQuery<Component[], AxiosError>("components", getComponents);

  let projects: string[] = [];
  let components: string[] = [];

  $: projects = $projectsResult.data?.map((p) => p.name) || [];
  $: components = $componentsResult.data?.map((p) => p.name) || [];

  // Experience: always have at least one empty row
  $: {
    if (allocations.length === 0) {
      newAllocation(null, null);
    }
  }

  function validate() {
    allocations.forEach((alloc) => {
      if (!projects.includes(alloc.project)) {
        throw Error(`Invalid Project: ${alloc.project}`);
      }
      if (!components.includes(alloc.component)) {
        throw Error(`Invalid Component: ${alloc.component}`);
      }
    });
    let totalPercent = allocations.reduce((a, b) => a + Number(b.percent), 0);
    if (totalPercent !== 100) {
      throw Error(`Total allocation: ${totalPercent}%`);
    }
  }

  const queryClient = useQueryClient();
  const updateMutation = useMutation<void, AxiosError, NewResourceAllocationPretty[]>(
    (allocations) => {
      validate();
      const newAllocations: NewResourceAllocation[] = allocations
        .filter((a) => Number(a.percent) > 0)
        .map((a) => ({
          project_id: $projectsResult.data.find((p) => p.name == a.project)?.id,
          component_id: $componentsResult.data.find((c) => c.name == a.component)?.id,
          percent: Number(a.percent),
        }));
      return putAllocations(resource.id, date, newAllocations);
    },
    {
      onSuccess: () => {
        queryClient.invalidateQueries(["alloc", dateToYMD(date)]);
        close();
      },
    }
  );

  const deleteMutation = useMutation<void, AxiosError>(
    () => deleteResourceAllocations(resource.id, date),
    {
      onSuccess: () => {
        queryClient.invalidateQueries(["alloc", dateToYMD(date)]);
        close();
      },
    }
  );


  function newAllocation(project: string, component: string) {
    let percent = allocations.reduce((a, b) => a + Number(b.percent), 0);
    allocations.push({ project, component, percent: 100 - percent });
    allocations = allocations;
  }

  function removeAllocation(index: number) {
    allocations.splice(index, 1);
    allocations = allocations;
  }

</script>

<div class="modal" class:is-active={is_active}>
  <div class="modal-background" on:click={close} />
  <div class="modal-card">
    <header class="modal-card-head">
      <p class="modal-card-title">{resource?.name} - week of {dateToYMD(date)}</p>
      <button class="delete" aria-label="close" on:click={close} />
    </header>
    <div class="modal-card-body" style="min-height:200px;">
      {#each allocations as allocation, index}
        <div class="field is-horizontal">
          <div class="field-body">
            <div class="field" style="width:300px;">
              <AutoComplete
                name="project"
                placeholder="Project"
                id={`project-select-${index}`}
                data={projects}
                bind:value={allocation.project}
              />
            </div>
            <div class="field" style="width:120px;">
              <AutoComplete
                name="component"
                placeholder="Component"
                id={`component-select-${index}`}
                data={components}
                bind:value={allocation.component}
              />
            </div>
            <div class="field" style="width:40px;">
              <span class="control has-icons-right">
                <input
                  class="input is-small"
                  type="number"
                  class:is-danger={allocation.percent < 0 || allocation.percent > 100}
                  placeholder="100"
                  bind:value={allocation.percent}
                />
                <a href="/" class="icon is-right is-small" tabindex="-1">
                  <span class="fas fa-percent" />
                </a></span
              >
            </div>

            <div>
              <button
                class="delete is-small"
                on:click={() => removeAllocation(index)}
              ></button>
            </div>
          </div>
        </div>
      {/each}

      <div class="field is-horizontal">
        <div class="field-body">
          <div class="field">
            <div class="control">
              <button class="button is-primary is-small" on:click={() => newAllocation(null, null)}>
                <span class="fas fa-plus" />
                &nbsp; Add Allocation
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    <footer class="modal-card-foot is-justify-content-space-between">
      <button class="button is-danger" on:click={() => $deleteMutation.mutate()}>
        Delete
      </button>
      <button class="button is-success" on:click={() => $updateMutation.mutate(allocations)}>
        Save changes
      </button>
    </footer>
  </div>
</div>

<style>
  .modal,
  .modal-card,
  .modal-card-body {
    overflow: visible;
  }
</style>
