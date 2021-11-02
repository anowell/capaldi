import axios, { AxiosError, AxiosResponse } from 'axios';

export interface User {
    id: number,
    email: string,
}

export async function postSession(email: string): Promise<User> {
    const { data } = await axios.post<User>(`/api/session`);
    return data;
}