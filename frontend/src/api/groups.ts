import axios, { AxiosError, AxiosResponse } from 'axios';

export interface Group {
    id:number,
    name:string,
}

export async function getGroups(): Promise<Group[]> {
    const { data } = await axios.get<Group[]>('/api/groups');
    return data;
}

export async function getGroup(id: number): Promise<Group> {
    const { data } = await axios.get<Group>(`/api/groups/${id}`);
    return data;
}