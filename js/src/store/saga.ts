import { SagaIterator } from 'redux-saga';
import { call, put, takeLatest } from 'redux-saga/effects';
import Registry from '../Registry';
import { Actions, PrefetchAction } from '../types';
import { failPrefetch, finishPrefetch, INDEX_THREAD, SHOW_THREAD, startPrefetch, updateThreadBody, updateThreadIndex } from './actions';

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

function showThreadCall ({ payload: { params: { thread_id: threadId } } }: PrefetchAction): Promise<any> {
  return Registry.api.showThread({ threadId });
}

function* showThread (action: ReturnType<Actions['showThread']>): SagaIterator {
  yield put(startPrefetch());

  try {
    const payload = yield call(showThreadCall, action);
    yield put(updateThreadBody(payload));
    yield put(finishPrefetch());
    action.payload.done();
  } catch (e) {
    yield put(failPrefetch(e));
    action.payload.failed(e);
  }
}

export default function* saga (): SagaIterator {
  yield takeLatest(INDEX_THREAD, indexThread);
  yield takeLatest(SHOW_THREAD, showThread);
}
