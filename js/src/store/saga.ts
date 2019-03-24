import { SagaIterator } from 'redux-saga';
import { call, put, takeLatest, select } from 'redux-saga/effects';
import Registry from '../Registry';
import { Actions, AppState } from '../types';
import { CREATE_MESSAGE, CREATE_THREAD, failPrefetch, finishPrefetch, INDEX_THREAD, resetMessage, resetThread, SHOW_THREAD, startPrefetch, updateThreadBody, updateThreadIndex } from './actions';

function indexThreadCall (_: any): Promise<any> {
  return Registry.api.indexThreads();
}

function* indexThread (action: ReturnType<Actions['indexThread']>): SagaIterator {
  yield put(startPrefetch());

  try {
    const payload = yield call(indexThreadCall, action);
    yield put(updateThreadIndex(payload));
    yield put(finishPrefetch());
    action.payload.done();
  } catch (e) {
    yield put(failPrefetch(e));
    action.payload.failed(e);
  }
}

function showThreadCall ({ threadId }): Promise<any> {
  return Registry.api.showThread({ threadId });
}

function* showThread (action: ReturnType<Actions['showThread']>): SagaIterator {
  yield put(startPrefetch());

  try {
    const payload = yield call(showThreadCall, { threadId: action.payload.params.thread_id });
    yield put(updateThreadBody(payload));
    yield put(finishPrefetch());
    action.payload.done();
  } catch (e) {
    yield put(failPrefetch(e));
    action.payload.failed(e);
  }
}

function createThreadCall ({ payload: { title, message } }): Promise<any> {
  return Registry.api.createThread({ title, message });
}

function* createThread (action: ReturnType<Actions['createThread']>): SagaIterator {
  const state: AppState = yield select();
  yield put(startPrefetch());
  try {
    const threadId = yield call(createThreadCall, { payload: state.threadParams });
    Registry.history.push(`threads/${threadId}`);
    yield put(resetThread());
  } catch (e) {
    console.error(e);
  }
}

function createMessageCall ({ payload: { threadId, message } }): Promise<any> {
  return Registry.api.createMessage({ threadId, message });
}

function* createMessage (action: ReturnType<Actions['createMessage']>): SagaIterator {
  const state: AppState = yield select();
  yield put(startPrefetch());
  try {
    yield call(createMessageCall, { payload: { ...action.payload, ...state.messageParams } });
    yield put(resetMessage());
    const payload = yield call(showThreadCall, { threadId: action.payload.threadId });
    yield put(updateThreadBody(payload));
  } catch (e) {
    console.error(e);
  }
}

export default function* saga (): SagaIterator {
  yield takeLatest(INDEX_THREAD, indexThread);
  yield takeLatest(SHOW_THREAD, showThread);
  yield takeLatest(CREATE_THREAD, createThread);
  yield takeLatest(CREATE_MESSAGE, createMessage);
}
