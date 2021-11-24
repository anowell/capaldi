import axios from 'axios';
import { fmtDate } from '../util';

export interface ProjectAggregate {
    start_date: Date,
    project_id: number,
    resources: number,
}

export async function getProjectAggregates(date: Date): Promise<ProjectAggregate[]> {
    const ymd = fmtDate(date);
    const { data } = await axios.get<ProjectAggregate[]>(`/api/aggregates/projects?start=${ymd}`);
    return data;
}

