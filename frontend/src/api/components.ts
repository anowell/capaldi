import axios from 'axios';

export interface Component {
    id: number,
    name: string,
};

export async function getComponents(): Promise<Component[]> {
    const { data } = await axios.get<Component[]>('/api/components');
    return data;
}
