import axios from 'axios';
import { ThreadBody, ThreadIndex } from './types';

const boardId = '1553139054.697814735s_00a175d3-4248-4f30-bfd1-91b880510d83';
const host = 'http://localhost:3000';
const baseURL = `${host}/api/b/${boardId}`;

const ax = axios.create({
  baseURL,
  timeout: 1000,
});

export function wait (n): Promise<void> {
  return new Promise(r => setInterval(r, n));
}

async function indexThreads (): Promise<ThreadIndex> {
  const { data: { summaries } } = await ax.get('');
  return { summaries };
}

async function createThread ({ title, message }): Promise<string> {
  const { data } = await ax.post('t', { title, message });
  return data;
}

async function createMessage ({ threadId, message }): Promise<string> {
  const { data } = await ax.post(`t/${threadId}/m`, { message });
  return data;
}

async function showThread ({ threadId }): Promise<ThreadBody> {
  const { data } = await ax.get(`t/${threadId}`);

  return { ...data, id: threadId };
}

const api = {
  indexThreads,
  showThread,
  createThread,
  createMessage,
};

export type Api = typeof api;
export default function createApi () {
  return api;
}
