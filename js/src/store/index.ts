import { applyMiddleware, createStore as createReduxStore, Store } from 'redux';
import createSagaMiddleware from 'redux-saga';
import { AppState } from '../types';
import messengerApp from './reducers';
import saga from './saga';

export default function createStore (initialState = {}): Store<AppState> {
  const sagaMiddleware = createSagaMiddleware();
  const store = createReduxStore(
    messengerApp,
    initialState,
    applyMiddleware(sagaMiddleware),
  );
  sagaMiddleware.run(saga);

  return store;
}
