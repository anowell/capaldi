import axios, { AxiosError, AxiosResponse } from 'axios';

export interface Allocation {
    id: number,
    start_date: Date,
    resource_id: number,
    project_id: number,
    component_id: number,
    percent: number,
}

// export interface AllocationView {
//     project: string,
//     component: string,
//     percent: number,
//     start_date: Date,
// }

export interface ResourceAllocation {
    id: number,
    project_id: number,
    component_id: number,
    percent: number,
}

export type AllocationMap = Record<string, Record<string, ResourceAllocation[]>>;

export interface NewResourceAllocation {
    project_id: number,
    component_id: number,
    percent: number,
}

export async function getAllocations(date: Date): Promise<AllocationMap> {
    const { data } = await axios.get<AllocationMap>(`/api/allocations?date=${date}`);
    return data;
}

export async function getResourceAllocations(resource_id: number, date: Date): Promise<Allocation[]> {
    const { data } = await axios.get<Allocation[]>(`/api/resources/${resource_id}/allocations`);
    return data;
}

export async function putAllocations(resource_id: number, date: Date, allocations: NewResourceAllocation[]): Promise<any> {
    const { data } = await axios.put(`/api/resources/${resource_id}/allocations/${date}`, allocations);
    return data;
}
