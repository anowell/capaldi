<script lang="ts">
  import type { AxiosError } from "axios";
  import { Group, getGroups } from "../api/groups";
  import { useQuery } from "@sveltestack/svelte-query";
  import AllocationModal from "../components/AllocationModal.svelte";
  import ResourceModal from "../components/ResourceModal.svelte";

  const groupsResult = useQuery<Group[], AxiosError>("groups", getGroups);

  let alloc_modal_active = false;
  let alloc_modal_resource_id;
  let alloc_modal_date;
  function editAllocations(resource_id: number, date: Date) {
    alloc_modal_resource_id = resource_id;
    alloc_modal_date = date;
    alloc_modal_active = true;
  }

  let res_modal_active = false;
  let res_modal_group_id;
  function newResource(group_id: number) {
    res_modal_group_id = group_id;
    res_modal_active = true;
  }
</script>

<div>
  <AllocationModal bind:is_active={alloc_modal_active} />
  <ResourceModal
    bind:is_active={res_modal_active}
    group_id={res_modal_group_id}
  />
  {#if $groupsResult.status === "loading"}
    <span>Loading...</span>
  {:else if $groupsResult.status === "error"}
    <span>Error: {$groupsResult.error.message}</span>
  {:else}
    <div>
      {#each $groupsResult.data as group}
        <h3 class="subtitle">{group.name}</h3>
        <table class="table is-hoverable is-striped is-fullwidth">
          <thead>
            <tr>
              <!-- <td style="width: 50px;">&nbsp;</td> -->
              <th>Resource</th>
              <th style="width: 25%;">10.18</th>
              <th style="width: 25%;" class="is-selected">10.25</th>
              <th style="width: 25%;">11.1</th>
            </tr>
          </thead>
          <tbody>
            {#each group.resources as resource}
              <tr>
                <!-- <td><a href={`1/edit`}><i class="fas fa-edit"></i></a></td> -->
                <td>{resource.name}</td>
                <td
                  class="is-clickable"
                  on:click={() => editAllocations(1, null)}
                >
                  <div>
                    Component Stewardship: Bugs <span class="tag">80%</span>
                  </div>
                  <div>
                    Time Off <span class="tag">20%</span>
                  </div>
                </td>
                <td
                  class="is-clickable is-selected"
                  on:click={() => editAllocations(1, null)}
                >
                  <div>
                    Time Off <span class="tag">100%</span>
                  </div>
                </td>

                <td
                  class="is-clickable"
                  on:click={() => editAllocations(1, null)}
                >
                  <div>
                    Component Stewardship: Bugs <span class="tag">50%</span>
                  </div>
                  <div>
                    Capacity Allocation Visualization <span class="tag"
                      >50%</span
                    >
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
        <div class="field pb-6">
          <div class="control">
            <button
              class="button is-primary is-small"
              on:click={() => newResource(group.id)}
            >
              <span class="fas fa-plus" />
              &nbsp; New Resource
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
