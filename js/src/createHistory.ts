import { createBrowserHistory } from 'history';
import { EventEmitter } from 'events';
import { matchPath } from 'react-router';
import { CustomRouteProps, PrefetchPayload, ScrollPositionStore, StoredScrollPosition } from './types';

export const defaultScrollPosition: StoredScrollPosition = { x: 0, y: 0 };

type A = {
  window: any
  routes: CustomRouteProps[]
  pathname: string
};

export default function createHistory ({ window, routes, pathname }: A) {
  const baseHistory = createBrowserHistory();
  const delayListener = new EventEmitter();
  const scrollPositions: ScrollPositionStore = {};
  const location = { pathname } as any;

  let idNow = 0;
  let currentId = 0;

  function storePosition () {
    const { scrollX: x, scrollY: y } = window;
    scrollPositions[currentId] = { x, y };
  }

  function scroll (id) {
    const { x, y } = scrollPositions[id] || defaultScrollPosition;
    setTimeout(() => window.scrollTo(x, y));
  }

  function prefetch ({ to, done, failed }: PrefetchPayload): void {
    const matched = routes.find(c => !!matchPath(to, c));

    if (!matched || !matched.prefetch) {
      done();
      return;
    }

    matched.prefetch({ to, params: matched.analyser(to), done, failed });
  }


  function transit () {
    const to = window.location.pathname;
    const id = currentId;
    prefetch({
      to,
      done: () => {
        if (id !== currentId) {
          // moved to another page before preparation.
          return;
        }
        location.pathname = to;
        delayListener.emit('popstate');
        scroll(id);
      },
      failed: (e: Error) => {
        console.error(e);
      },
    });
  }

  function work (type, url) {
    storePosition();
    idNow += 1;
    currentId = idNow;

    window.history[`${type}State`]({ id: idNow }, `page${idNow}`, url);
    transit();
  }

  function listen (f: () => any): () => void {
    delayListener.addListener('popstate', f);
    return () => delayListener.removeListener('popstate', f);
  }

  const onPopState = ({ state: { id } }: PopStateEvent) => {
    storePosition();
    currentId = id;

    transit();
  };

  function push (url) {
    work('push', url);
  }

  function replace (url) {
    work('replace', url);
  }

  window.addEventListener('popstate', onPopState);

  const addition = {
    listen,
    push,
    replace,
    location,
    prefetch,
  };

  return Object.assign(baseHistory, addition);
}
