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

export type AllocationMap = Record<string, Allocation[]>;

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

export async function getAllocations(date: Date, all: boolean = false): Promise<AllocationMap> {
    const ymd = dateToYMD(date);
    const { data } = await axios.get<AllocationMap>(`/api/allocations?start=${ymd}&all=${all}`);
    return data;
}

export async function getResourceAllocations(resource_id: number, date: Date): Promise<Allocation[]> {
    const ymd = dateToYMD(date);
    const { data } = await axios.get<Allocation[]>(`/api/resources/${resource_id}/allocations?start=${ymd}`);
    return data;
}

export async function putAllocations(resource_id: number, date: Date, allocations: NewResourceAllocation[]): Promise<any> {
    const ymd = dateToYMD(date);
    const { data } = await axios.put(`/api/resources/${resource_id}/allocations/${ymd}`, allocations);
    return data;
}

export async function deleteResourceAllocations(resource_id: number, date: Date): Promise<void> {
    const ymd = dateToYMD(date);
    const { data } = await axios.delete(`/api/resources/${resource_id}/allocations/${ymd}`);
}

export async function getProjectAllocations(project_id: number, date: Date): Promise<Allocation[]> {
    const ymd = dateToYMD(date);
    const { data } = await axios.get<Allocation[]>(`/api/projects/${project_id}/allocations?start=${ymd}`);
    return data;
}