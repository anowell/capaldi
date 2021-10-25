<script lang="ts">
  import type { AxiosError } from "axios";
  import type { Resource, Group, getGroups } from "../api/groups";
  import { Project, getProjects } from "../api/projects";
  import { useQuery } from "@sveltestack/svelte-query";
  import AutoComplete from "./AutoComplete.svelte";

  // export let date: Date;
  // export let resource: Resource;
  export let is_active: boolean;

  let close = () => (is_active = false);


  const projectsResult = useQuery<Project[], AxiosError>(
    "projects",
    getProjects
  );
  let projects = [];
  $: {
    if ($projectsResult.data) {
      projects = $projectsResult.data.map((p) => p.name);
    }
  }

  // let allocationResult = useQuery<[], AxiosError>(`alloc-${resource.id}-${date}`, () =>
  //   getAllocations(resource.id, date)
  // );

  // function validate() {
  //   allocations.forEach((alloc) => {
  //     if(!projects.indexOf(alloc.project)) {
  //       console.log(`Invalid Project: ${alloc.project}`);
  //     }
  //     if(!components.indexOf(alloc.component)) {
  //       console.log(`Invalid Component: ${alloc.component}`);
  //     }
  //   });
  //   let totalPercent = allocations.reduce((a,b)=> a+Number(b.percent), 0);
  //   if( totalPercent !== 100) {
  //     console.log(`Total allocation: ${totalPercent}%`)
  //   }
  // }

  let allocations: { project: string; component: string; percent: number }[] =
    [];
  function newAllocation(project: string, component: string) {
    let percent = allocations.reduce((a,b)=> a+Number(b.percent), 0);
    allocations.push({ project, component, percent: 100-percent });
    allocations = allocations;
  }
  newAllocation(null, null);

  function removeAllocation(index: number) {
    allocations.splice(index, 1);
    allocations = allocations;
  }

  let components = ["Frontend", "Backend", "Infrastructure", "Tools"];
</script>

<div class="modal" class:is-active={is_active}>
  <div class="modal-background" on:click={close} />
  <div class="modal-card">
    <header class="modal-card-head">
      <p class="modal-card-title">Allocate Resource</p>
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
                  type="text"
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
              <button class="button is-danger is-small is-inverted" on:click={()=>removeAllocation(index)}>
                <span class="icon is-small">
                  <i class="fas fa-times" />
                </span>
              </button>
            </div>
          </div>
        </div>
      {/each}

      <div class="field is-horizontal">
        <div class="field-body">
          <div class="field">
            <div class="control">
              <button
                class="button is-primary is-small"
                on:click={() => newAllocation(null, null)}
              >
                <span class="fas fa-plus" />
                &nbsp; Add Allocation
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    <footer class="modal-card-foot">
      <button class="button is-success">Save changes</button>
      <button class="button" on:click={close}>Cancel</button>
    </footer>
  </div>
</div>

<style>
  .modal, .modal-card, .modal-card-body {
    overflow: visible;
  }
</style>