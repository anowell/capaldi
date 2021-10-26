import axios, { AxiosError, AxiosResponse } from 'axios';

export interface Group {
    id: number,
    name: string,
    resources: Resource[],
}

export interface Resource {
    id: number,
    name: string,
    // role_id: number,
}

export interface NewResource {
    name: string,
    role_id: number,
    is_fte: boolean,
}


export async function getGroups(): Promise<Group[]> {
    const { data } = await axios.get<Group[]>('/api/groups');
    return data;
}

export async function getGroup(id: number): Promise<Group> {
    const { data } = await axios.get<Group>(`/api/groups/${id}`);
    return data;
}

export async function postResource(group_id: number, resource: NewResource): Promise<void> {
    const { data } = await axios.post<any>(`/api/groups/${group_id}/resources`, resource);
}

// export async function putResource(group_id: number, resource_id: number, resource: NewResource): Promise<void> {
//     const { data } = await axios.post<any>(`/api/groups/${group_id}/resources/${resource_id}`, resource);
// }
