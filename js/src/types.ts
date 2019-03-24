import { RouteProps } from 'react-router';
import { Store } from 'redux';
import { Api } from './clientSideApi';
import * as actions from './store/actions';

export const defaultState = {
  transitionId: '',
  prefetchStatus: {} as PreFetchStatus,
  title: '',
  threadIndex: {} as ThreadIndex,
  threadBody: {} as ThreadBody,
  threadParams: { message: '', title: '' } as ThreadCreationParams,
  messageParams: { message: '' } as MessageCreationParams,
};

export type AppState = typeof defaultState;

export type Registry = {
  api: Api
  routes: CustomRouteProps[]
  store: Store<AppState>
}

export type CustomRouteProps = RouteProps & {
  analyser?: any
  prefetch?: (
    payload: RoutePrefetchPayload,
  ) => any
}

export type RoutePrefetchPayload = PrefetchPayload & {
  params: any,
}

export type PrefetchPayload = {
  to: string,
  done: () => any
  failed: (e: Error) => any
}

export type PrefetchAction = {
  type: string
  payload: RoutePrefetchPayload
}

export type PreFetchStatus = {
  type: 'ready' | 'started' | 'finished' | 'failed'
  message: string
}

export type ThreadSummary = {
  id: string
  title: string
  count: number
  locked: boolean
}

export type ThreadIndex = {
  summaries: ThreadSummary[]
}

export type ThreadBody = {
  id: string
  title: string
  head: number
  tail: number
  messages: ThreadItem[]
}

export type ThreadItem = {
  index: number
  name: string
  email: string
  html: string
  mentioned: number[]
  mention: number[]
}

export type ThreadCreationParams = {
  title: string
  message: string
}

export type MessageCreationParams = {
  message: string
}

export type Actions = typeof actions
// export type ActionRT = { [N in keyof Actions]: ReturnType<Actions[N]> }


export type StoredScrollPosition = { x: number, y: number }
export type ScrollPositionStore = { [key: number]: StoredScrollPosition }
export type RoutingListener = (l: Location) => any
