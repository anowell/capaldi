<script lang="ts">
    import type { AxiosError } from "axios";
    import { type Resource, type Team, type NewResource, getTeams, postResource } from "../api/teams";
    import { type Project, getProjects } from "../api/projects";
    import { useQuery, useMutation, useQueryClient } from "@sveltestack/svelte-query";

    export let is_active: boolean;
    export let team_id: number;

    let error: string;
    let closeModal = () => {
        error = null;
        is_active = false;
    };
    let resource: NewResource = { name: "", role_id: 1, is_fte: true };

    const queryClient = useQueryClient();
    const createResource = useMutation<void, AxiosError, NewResource>(
        (resource) => postResource(team_id, resource),
        {
            onSuccess: () => {
                queryClient.invalidateQueries("teams");
                closeModal();
            },
        }
    );
</script>

<div class="modal" class:is-active={is_active}>
    <div class="modal-background" on:click={closeModal} />
    <div class="modal-card">
        <header class="modal-card-head">
            <p class="modal-card-title">Create Resource</p>
            <button class="delete" aria-label="close" on:click={closeModal} />
        </header>
        <div class="modal-card-body">
            <div class="field">
                <label class="label" for="resource-name">Name</label>
                <div class="control">
                    <input
                        id="resource-name"
                        class="input"
                        type="text"
                        placeholder="Name"
                        bind:value={resource.name}
                    />
                </div>
            </div>

            <div class="field">
                <label class="label" for="resource-role">Role</label>
                <div class="control">
                    <div class="select">
                        <select bind:value={resource.role_id} id="resource-role">
                            <option value={1}>TODO ROLE #1</option>
                            <option value={2}>TODO ROLE #2</option>
                        </select>
                    </div>
                </div>
            </div>

            <div class="field">
                <div class="control">
                    <label class="checkbox">
                        <input type="checkbox" bind:checked={resource.is_fte} />
                        Is FTE?
                    </label>
                </div>
            </div>
        </div>
        <footer class="modal-card-foot">
            <button
                type="submit"
                class="button is-success"
                on:click={() => $createResource.mutate(resource)}>Create</button
            >
            <button class="button" on:click={closeModal}>Cancel</button>
        </footer>
    </div>
</div>
