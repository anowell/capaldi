<script lang="ts">
  import type { AxiosError } from "axios";
  import { Group, getGroups } from "../api/groups";
  import { Project, getProjects } from "../api/projects";
  import { useQuery } from "@sveltestack/svelte-query";
  import AutoComplete from "./AutoComplete.svelte";
  let is_active = true;
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

  function validate() {
    let i = projects.indexOf("bison");
    let projectId = null;
    if (i >= 0) {
      projectId = $projectsResult.data[i].id;
    }
  }

  let allocations: { project: string; component: string; percent: number }[] =
    [];
  function newAllocation(project: string, component: string, percent: number) {
    allocations.push({ project, component, percent });
    allocations = allocations;
  }
  newAllocation(null, null, 100);

  let components = ["Frontend", "Backend", "Infrastructure", "Tools"];
</script>

<div class="modal" class:is-active={is_active}>
  <div class="modal-background" />
  <div class="modal-card">
    <header class="modal-card-head">
      <p class="modal-card-title">Allocate Resource</p>
      <button class="delete" aria-label="close" on:click={close} />
    </header>
    <div class="modal-card-body is-clipped" style="min-height:400px;">
      {#each allocations as allocation, index}
        <div class="field is-horizontal">
          <div class="field-body">
            <div class="field">
              <AutoComplete
                name="project"
                placeholder="Project"
                id={`project-select-${index}`}
                data={projects}
                bind:value={allocation.project}
              />
            </div>
            <div class="field">
              <AutoComplete
                name="component"
                placeholder="Component"
                id={`component-select-${index}`}
                data={components}
                bind:value={allocation.component}
              />
            </div>
            <div class="field">
              <span class="control has-icons-right">
                <input
                  class="input"
                  type="text"
                  placeholder="100"
                  bind:value={allocation.percent}
                />
                <a href="/" class="icon is-right is-small" tabindex="-1">
                  <span class="fas fa-percent" />
                </a></span
              >
            </div>
          </div>
        </div>
      {/each}

      <div class="field is-horizontal">
        <div class="field-body">
          <div class="field">
            <div class="control">
              <button
                class="button is-primary"
                on:click={() => newAllocation(null, null, 0)}
              >
                <ion-icon name="add" />
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
