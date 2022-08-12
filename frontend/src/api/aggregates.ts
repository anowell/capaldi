import axios from 'axios';
import { fmtDate } from '../util';

export interface ProjectAggregate {
    start_date: Date,
    project_id: number,
    resources: number,
}

export async function getProjectAggregates(start: Date, end: Date = undefined): Promise<ProjectAggregate[]> {
    if (end === undefined) {
        end = start
    }

    const { data } = await axios.get<ProjectAggregate[]>(`/api/aggregates/projects?start=${fmtDate(start)}&end=${fmtDate(end)}`);
    return data;
}
