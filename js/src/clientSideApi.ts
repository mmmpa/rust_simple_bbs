import { testThreadBodies, testThreadIndex } from './fixtures/threads';
import { ThreadBody, ThreadIndex } from './types';

function wait (n): Promise<void> {
  return new Promise(r => setInterval(r, n));
}

async function indexThreads (): Promise<ThreadIndex> {
  await wait(1000);
  return testThreadIndex;
}

async function showThread ({ threadId }): Promise<ThreadBody> {
  await wait(1000);

  return testThreadBodies[threadId] || Promise.reject();
}

const api = {
  indexThreads,
  showThread,
};

export type Api = typeof api
export default api;
