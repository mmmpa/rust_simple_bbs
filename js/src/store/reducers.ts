import { AppState, defaultState as ds } from '../types';
import { FAIL_PREFETCH, FINISH_PREFETCH, RESET_MESSAGE_PARAMS, RESET_THREAD_PARAMS, START_PREFETCH, UPDATE_MESSAGE_PARAMS, UPDATE_THREAD_BODY, UPDATE_THREAD_INDEX, UPDATE_THREAD_PARAMS, UPDATE_TRANSITION_ID } from './actions';

function title (state = ds.title, action): AppState['title'] {
  switch (action.type) {
  case UPDATE_THREAD_BODY:
    return action.payload;
  default:
    return state;
  }
}

function threadBody (state = ds.threadBody, action): AppState['threadBody'] {
  switch (action.type) {
  case UPDATE_THREAD_BODY:
    return action.payload;
  default:
    return state;
  }
}

function threadIndex (state = ds.threadIndex, action): AppState['threadIndex'] {
  switch (action.type) {
  case UPDATE_THREAD_INDEX:
    return action.payload;
  default:
    return state;
  }
}

function transitionId (state = ds.transitionId, action): AppState['transitionId'] {
  switch (action.type) {
  case UPDATE_TRANSITION_ID:
    return action.payload;
  default:
    return state;
  }
}

function prefetchStatus (state = ds.prefetchStatus, action): AppState['prefetchStatus'] {
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

function threadParams (state = ds.threadParams, action): AppState['threadParams'] {
  console.log(action);
  switch (action.type) {
  case UPDATE_THREAD_PARAMS:
    return action.payload;
  case RESET_THREAD_PARAMS:
    return { ...ds.threadParams };
  default:
    return state;
  }
}

function messageParams (state = ds.messageParams, action): AppState['messageParams'] {
  switch (action.type) {
  case UPDATE_MESSAGE_PARAMS:
    return action.payload;
  case RESET_MESSAGE_PARAMS:
    return { ...ds.messageParams };
  default:
    return state;
  }
}

export default function app (state = ds, action): AppState {
  return {
    title: title(state.title, action),
    transitionId: transitionId(state.transitionId, action),
    prefetchStatus: prefetchStatus(state.prefetchStatus, action),
    threadIndex: threadIndex(state.threadIndex, action),
    threadBody: threadBody(state.threadBody, action),
    threadParams: threadParams(state.threadParams, action),
    messageParams: messageParams(state.messageParams, action),
  };
}
