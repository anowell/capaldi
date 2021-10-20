import axios, { AxiosError, AxiosResponse } from 'axios';

export interface Project {
    id:number,
    name:string,
}

export async function getProjects(): Promise<Project[]> {
    const { data } = await axios.get<Project[]>('/api/projects');
    return data;
}

export async function getProject(id: number): Promise<Project> {
    const { data } = await axios.get<Project>(`/api/projects/${id}`);
    return data;
}