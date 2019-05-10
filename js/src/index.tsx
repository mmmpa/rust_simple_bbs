import React from 'react';
import * as ReactDOM from 'react-dom';
import './index.styles.sass';
import createApi from './clientSideApi';
import createApp from './createApp';
import createHistory from './createHistory';
import createRoutes from './createRoutes';
import createStore from './store';
import Registry from './Registry';

function start (window): void {
  const store = createStore();
  const [routes, AppRoutes] = createRoutes({ store });
  const { pathname } = window.location;
  const history = createHistory({ window, routes, pathname });

  Registry.api = createApi();
  Registry.history = history;
  Registry.routes = routes;
  Registry.store = store;

  const App = createApp({
    history,
    store,
    AppRoutes,
  });

  history.prefetch({
    to: pathname,
    done: () => {
      ReactDOM.render(
        <App />,
        document.getElementById('app'),
      );
    },
    failed: (e: Error) => {
      console.error(e);
    },
  });
}

start(window);
