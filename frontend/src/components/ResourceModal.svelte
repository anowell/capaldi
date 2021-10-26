<script lang="ts">
    import type { AxiosError } from "axios";
    import {
        Resource,
        Group,
        getGroups,
        postResource,
        NewResource,
    } from "../api/groups";
    import { Project, getProjects } from "../api/projects";
    import {
        useQuery,
        useMutation,
        useQueryClient,
    } from "@sveltestack/svelte-query";

    export let is_active: boolean;
    export let group_id: number;

    let error: string;
    let close = () => {
        error = null;
        is_active = false;
    };
    let resource: NewResource = { name: "", role_id: 1, is_fte: true };

    const queryClient = useQueryClient();
    const createResource = useMutation<void, AxiosError, NewResource>(
        (resource) => postResource(group_id, resource),
        {
            onSuccess: () => {
                queryClient.invalidateQueries("groups");
                close();
            },
        }
    );

</script>

<div class="modal" class:is-active={is_active}>
    <div class="modal-background" on:click={close} />
    <div class="modal-card">
        <header class="modal-card-head">
            <p class="modal-card-title">Create Resource</p>
            <button class="delete" aria-label="close" on:click={close} />
        </header>
        <div class="modal-card-body">
            <div class="field">
                <label class="label">Name</label>
                <div class="control">
                    <input
                        class="input"
                        type="text"
                        placeholder="Name"
                        bind:value={resource.name}
                    />
                </div>
            </div>

            <div class="field">
                <label class="label">Role</label>
                <div class="control">
                    <div class="select">
                        <select bind:value={resource.role_id}>
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
                on:click={$createResource.mutate(resource)}>Create</button
            >
            <button class="button" on:click={close}>Cancel</button>
        </footer>
    </div>
</div>
