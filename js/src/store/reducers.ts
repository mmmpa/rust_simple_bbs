import { AppState, PreFetchStatus, ThreadBody, ThreadIndex } from '../types';
import { FAIL_PREFETCH, FINISH_PREFETCH, START_PREFETCH, UPDATE_THREAD_BODY, UPDATE_THREAD_INDEX, UPDATE_TRANSITION_ID } from './actions';

function threadBody (state: ThreadBody = {} as ThreadBody, action): ThreadBody {
  switch (action.type) {
  case UPDATE_THREAD_BODY:
    return action.payload;
  default:
    return state;
  }
}

function threadIndex (state: ThreadIndex = {} as ThreadIndex, action): ThreadIndex {
  switch (action.type) {
  case UPDATE_THREAD_INDEX:
    return action.payload;
  default:
    return state;
  }
}

function transitionId (state: string, action): string {
  switch (action.type) {
  case UPDATE_TRANSITION_ID:
    return action.payload;
  default:
    return state;
  }
}

function prefetchStatus (state: PreFetchStatus = {} as PreFetchStatus, action): PreFetchStatus {
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
}

export default function app (state = {} as AppState, action): AppState {
  return {
    title: 'hoge',
    transitionId: transitionId(state.transitionId, action),
    prefetchStatus: prefetchStatus(state.prefetchStatus, action),
    threadIndex: threadIndex(state.threadIndex, action),
    threadBody: threadBody(state.threadBody, action),
  };
}
