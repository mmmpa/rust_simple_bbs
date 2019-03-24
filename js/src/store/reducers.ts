import { AppState, MessageCreationParams, PreFetchStatus, ThreadBody, ThreadCreationParams, ThreadIndex } from '../types';
import { FAIL_PREFETCH, FINISH_PREFETCH, START_PREFETCH, UPDATE_MESSAGE_PARAMS, UPDATE_THREAD_BODY, UPDATE_THREAD_INDEX, UPDATE_THREAD_PARAMS, UPDATE_TRANSITION_ID } from './actions';

const ds = {
  transitionId: '',
  prefetchStatus: {} as PreFetchStatus,
  title: '',
  threadIndex: {} as ThreadIndex,
  threadBody: {} as ThreadBody,
  threadParams: {} as ThreadCreationParams,
  messageParams: {} as MessageCreationParams,
};

export const defaultState = ds;

const reducers: { [K in keyof AppState]: (state: AppState[K], action: any) => AppState[K] } = {
  title: (state = ds.title, action) => {
    switch (action.type) {
    case UPDATE_THREAD_BODY:
      return action.payload;
    default:
      return state;
    }
  },

  threadBody: (state = ds.threadBody, action) => {
    switch (action.type) {
    case UPDATE_THREAD_BODY:
      return action.payload;
    default:
      return state;
    }
  },

  threadIndex: (state = ds.threadIndex, action) => {
    switch (action.type) {
    case UPDATE_THREAD_INDEX:
      return action.payload;
    default:
      return state;
    }
  },

  transitionId: (state = ds.transitionId, action) => {
    switch (action.type) {
    case UPDATE_TRANSITION_ID:
      return action.payload;
    default:
      return state;
    }
  },

  prefetchStatus: (state = ds.prefetchStatus, action) => {
    switch (action.type) {
    case START_PREFETCH:
      return { type: 'started', message: action.payload };
    case FINISH_PREFETCH:
      return { type: 'finished', message: action.payload };
    case FAIL_PREFETCH:
      return { type: 'failed', message: action.payload };
    default:
      return state;
    }
  },

  threadParams: (state = ds.threadParams, action) => {
    switch (action.type) {
    case UPDATE_THREAD_PARAMS:
      return action.payload;
    default:
      return state;
    }
  },

  messageParams: (state = ds.messageParams, action) => {
    switch (action.type) {
    case UPDATE_MESSAGE_PARAMS:
      return action.payload;
    default:
      return state;
    }
  },
};

export default function app (state = {} as AppState, action): AppState {
  return {
    title: reducers.title(state.title, action),
    transitionId: reducers.transitionId(state.transitionId, action),
    prefetchStatus: reducers.prefetchStatus(state.prefetchStatus, action),
    threadIndex: reducers.threadIndex(state.threadIndex, action),
    threadBody: reducers.threadBody(state.threadBody, action),
    threadParams: reducers.threadParams(state.threadParams, action),
    messageParams: reducers.messageParams(state.messageParams, action),
  };
}
