import * as React from 'react';
import * as ReactDOM from 'react-dom';
import createApp from './createApp';
import './index.styles.sass';
import createApi from './clientSideApi';
import createRoutes from './createRoutes';
import createHistory from './createHistory';
import Registry from './Registry';
import createStore from './store';

function start (window): void {
  const store = createStore();
  const [routes, AppRoutes] = createRoutes({ store });

  Registry.routes = routes;
  Registry.store = store;
  Registry.api = createApi();

  const { pathname } = window.location;
  const history = createHistory({ window, routes, pathname });

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
