import { AnyAction } from 'redux';
import { PrefetchAction, RoutePrefetchPayload } from '../types';

export const START_PREFETCH = 'START_PREFETCH';
export const FINISH_PREFETCH = 'FINISH_PREFETCH';
export const FAIL_PREFETCH = 'FAIL_PREFETCH';
export const INDEX_THREAD = 'INDEX_THREAD';
export const CREATE_THREAD = 'CREATE_THREAD';
export const CREATE_MESSAGE = 'CREATE_MESSAGE';
export const UPDATE_THREAD_INDEX = 'UPDATE_THREAD_INDEX';
export const SHOW_THREAD = 'SHOW_THREAD';
export const UPDATE_THREAD_BODY = 'UPDATE_THREAD_BODY';
export const UPDATE_TRANSITION_ID = 'UPDATE_TRANSITION_ID';

export const UPDATE_THREAD_PARAMS = 'UPDATE_THREAD_PARAMS';
export const RESET_THREAD_PARAMS = 'RESET_THREAD_PARAMS';
export const UPDATE_MESSAGE_PARAMS = 'UPDATE_MESSAGE_PARAMS';
export const RESET_MESSAGE_PARAMS = 'RESET_MESSAGE_PARAMS';

export function updateTransitionId (payload): AnyAction {
  return { type: UPDATE_TRANSITION_ID, payload };
}

export function startPrefetch (): AnyAction {
  return { type: START_PREFETCH };
}

export function finishPrefetch (): AnyAction {
  return { type: FINISH_PREFETCH };
}

export function failPrefetch (payload): AnyAction {
  return { type: FAIL_PREFETCH, payload };
}

export function indexThread (payload: RoutePrefetchPayload): PrefetchAction {
  return { type: INDEX_THREAD, payload };
}

export function updateThreadIndex (payload): AnyAction {
  return { type: UPDATE_THREAD_INDEX, payload };
}

export function showThread (payload: RoutePrefetchPayload): PrefetchAction {
  return { type: SHOW_THREAD, payload };
}

export function createThread (payload) {
  return { type: CREATE_THREAD, payload };
}

export function createMessage (payload) {
  return { type: CREATE_MESSAGE, payload };
}

export function updateThread (payload) {
  return { type: UPDATE_THREAD_PARAMS, payload };
}

export function updateMessage (payload) {
  return { type: UPDATE_MESSAGE_PARAMS, payload };
}

export function resetThread () {
  return { type: RESET_THREAD_PARAMS };
}

export function resetMessage () {
  return { type: RESET_MESSAGE_PARAMS };
}

export function updateThreadBody (payload): AnyAction {
  return { type: UPDATE_THREAD_BODY, payload };
}
