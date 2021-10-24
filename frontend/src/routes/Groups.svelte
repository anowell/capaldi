<script lang="ts">
    import type { AxiosError } from "axios";
    import { Group, getGroups } from '../api/groups';
    import { useQuery } from '@sveltestack/svelte-query';
    import Allocation from '../components/Allocation.svelte';

    const groupsResult = useQuery<Group[], AxiosError>('groups', getGroups);
</script>

<div>
    <Allocation />
    {#if $groupsResult.status === 'loading'}
        <span>Loading...</span>
    {:else if $groupsResult.status === 'error'}
        <span>Error: {$groupsResult.error.message}</span>
    {:else}
        <div>
            {#each $groupsResult.data as group}
              <h3 class="subtitle">{group.name}</h3>

            {/each}

            <table class="table is-hoverable is-striped is-fullwidth">
                <thead>
                    <tr>
                        <td style="width: 50px;">&nbsp;</td>
                        <th>Resource</th>
                        <th>10.18</th>
                        <th class="is-selected">10.25</th>
                        <th>11.1</th>
                    </tr>
                </thead>
                <tbody>
                        <tr>
                            <td><a href="#"><ion-icon name="create"></ion-icon></a></td>
                            <td>Rebekah Martin</td>
                            <td>
                                <div>Time Off <span class="tag">100%</span></div>
                            </td>
                            <td class="is-selected">

                            </td>
                            <td>

                            </td>
                        </tr>
                        <tr>
                            <td><a href="#"><ion-icon name="create"></ion-icon></a></td>
                            <td>Samira Manoja </td>
                            <td>
                                <div>Component Stewardship: Bugs <span class="tag">50%</span></div>
                                <div>Capacity Allocation Visualization <span class="tag">50%</span></div>
                            </td>
                            <td class="is-selected">
                                <div>Capacity Allocation Visualization <span class="tag">100%</span></div>
                            </td>
                            <td>
                                <div>Capacity Allocation Visualization <span class="tag">100%</span></div>
                            </td>
                        </tr>
                        <tr>
                            <td><a href="#"><ion-icon name="create"></ion-icon></a></td>
                            <td>Colin Layton</td>
                            <td>
                                <div>Component Stewardship: Bugs <span class="tag">20%</span></div>
                                <div>Port package to Debian <span class="tag">80%</span></div>
                                <div>Kubernetes Upversion <span class="tag">100%</span></div>
                            </td>
                            <td class="is-selected">
                                <div>Kubernetes Upversion <span class="tag">50%</span></div>
                            </td>
                            <td>
                                <div>Time Off <span class="tag">100%</span></div>
                            </td>
                        </tr>
                </tbody>
            </table>
    
        </div>
    {/if}
</div>
