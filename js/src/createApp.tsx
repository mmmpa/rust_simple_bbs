import * as React from 'react';
import { Provider } from 'react-redux';
import { Router } from 'react-router-dom';
import GlobalFooter from './components/GlobalFooter';
import GlobalHeader from './components/GlobalHeader';

export default function createApp ({ history, store, AppRoutes }): () => JSX.Element {
  return function App () {
    return (
      <Router history={history}>
        <Provider store={store}>
          <div className='container'>
            <GlobalHeader />
            {AppRoutes}
            <GlobalFooter />
          </div>
        </Provider>
      </Router>
    );
  };
}
