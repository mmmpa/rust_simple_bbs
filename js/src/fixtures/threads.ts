import { ThreadBody, ThreadIndex } from '../types';

function array (n): Array<number> {
  return Array(n).join(',').split(',').map((_, i) => i);
}

const testThreads: { [key: string]: any } = {
  abc: {
    title: 'tread A',
    items: array(33).map(i => (
      { index: i, name: '', email: '', comment: `thread A comment ${i}`, mentioned: [], mention: [] }
    )),
  },
  def: {
    title: 'tread B',
    items: array(1).map(i => (
      { index: i, name: '', email: '', comment: `thread B comment ${i}`, mentioned: [], mention: [] }
    )),
  },
  ghi: {
    title: 'tread C',
    items: array(100).map(i => (
      { index: i, name: '', email: '', comment: `thread C comment ${i}`, mentioned: [], mention: [] }
    )),
  },
  jkl: {
    title: 'tread D',
    items: array(20).map(i => (
      { index: i, name: '', email: '', comment: `thread D comment ${i}`, mentioned: [], mention: [] }
    )),
  },
};


export const testThreadIndex: ThreadIndex = {
  summaries: Object.keys(testThreads).map((id) => {
    const { title, items } = testThreads[id];

    return {
      id,
      title,
      count: items.length,
      locked: items.length >= 100,
    };
  }),
};

export const testThreadBodies: { [key: string]: ThreadBody } = Object.keys(testThreads).reduce((a, id) => {
  const { title, items } = testThreads[id];

  a[id] = {
    title,
    head: 0,
    tail: items.length - 1,
    items,
  };

  return a;
}, {});
