import { SagaIterator } from 'redux-saga';
import { call, put, takeLatest } from 'redux-saga/effects';
import Registry from '../Registry';
import { Actions } from '../types';
import { CREATE_MESSAGE, CREATE_THREAD, failPrefetch, finishPrefetch, INDEX_THREAD, SHOW_THREAD, startPrefetch, updateThreadBody, updateThreadIndex } from './actions';

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
  yield put(startPrefetch());
  try {
    yield call(createThreadCall, action);
  } catch (e) {
    console.error(e);
  }
}

function createMessageCall ({ payload: { threadId, message } }): Promise<any> {
  return Registry.api.createMessage({ threadId, message });
}

function* createMessage (action: ReturnType<Actions['createMessage']>): SagaIterator {
  yield put(startPrefetch());
  try {
    yield call(createMessageCall, action);
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
