import axios from 'axios';
import { dateToYMD } from '../util';

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

export interface NewResourceAllocationPretty {
    project: string,
    component: string,
    percent: number,
}

export async function getAllocations(date: Date): Promise<AllocationMap> {
    const ymd = dateToYMD(date);
    const { data } = await axios.get<AllocationMap>(`/api/allocations?from=${ymd}`);
    return data;
}

export async function getResourceAllocations(resource_id: number, date: Date): Promise<Allocation[]> {
    const ymd = dateToYMD(date);
    const { data } = await axios.get<Allocation[]>(`/api/resources/${resource_id}/allocations?from=${ymd}`);
    return data;
}

export async function putAllocations(resource_id: number, date: Date, allocations: NewResourceAllocation[]): Promise<any> {
    const ymd = dateToYMD(date);
    const { data } = await axios.put(`/api/resources/${resource_id}/allocations/${ymd}`, allocations);
    return data;
}
