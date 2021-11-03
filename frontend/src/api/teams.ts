import axios from 'axios';

export interface Team {
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


export async function getTeams(): Promise<Team[]> {
    const { data } = await axios.get<Team[]>('/api/teams');
    return data;
}

export async function getTeam(id: number): Promise<Team> {
    const { data } = await axios.get<Team>(`/api/teams/${id}`);
    return data;
}

export async function postResource(team_id: number, resource: NewResource): Promise<void> {
    const { data } = await axios.post<any>(`/api/teams/${team_id}/resources`, resource);
}

// export async function putResource(team_id: number, resource_id: number, resource: NewResource): Promise<void> {
//     const { data } = await axios.post<any>(`/api/teams/${team_id}/resources/${resource_id}`, resource);
// }
