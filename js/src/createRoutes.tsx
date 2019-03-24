import * as React from 'react';
import { Route } from 'react-router-dom';
import * as pathToRegexp from 'path-to-regexp';
import LandingPage from './components/LandingPage';
import ThreadBody from './components/ThreadBody';
import ThreadCreation from './components/ThreadCreation';
import ThreadIndex from './components/ThreadIndex';
import { indexThread, showThread } from './store/actions';
import { CustomRouteProps } from './types';

function insertAnalyser (props: CustomRouteProps): CustomRouteProps {
  const keys = [];
  const reg = pathToRegexp(props.path || '', keys);
  props.analyser = (to: string) => {
    const matched = reg.exec(to);
    if (!matched) {
      return {};
    }
    return keys.reduce((a, { name }, i) => {
      a[name] = matched[i + 1];
      return a;
    }, {} as any);
  };

  return props;
}

export default function createRoutes ({ store }): [CustomRouteProps[], Route[]] {
  const routes: CustomRouteProps[] = [
    {
      path: '/',
      component: LandingPage,
      exact: true,
      prefetch: payload => store.dispatch(indexThread(payload)),
    },
    {
      path: '/new_thread',
      exact: true,
      component: ThreadCreation,
    },
    {
      path: '/threads',
      exact: true,
      component: ThreadIndex,
    },
    {
      path: '/threads/:thread_id',
      exact: true,
      component: ThreadBody,
      prefetch: payload => store.dispatch(showThread(payload)),
    },
    {
      path: '/threads/:thread_id/:comment_range',
      exact: true,
      component: ThreadBody,
      prefetch: payload => store.dispatch(showThread(payload)),
    },
  ].map(insertAnalyser);

  const AppRoutes = routes.map(c => (
    <Route {...c} />
  ));

  return [routes, AppRoutes];
}
