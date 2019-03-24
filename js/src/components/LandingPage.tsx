import * as React from 'react';
import ThreadIndexTable from './ThreadIndexTable';

export default function LandingPage (): JSX.Element {
  return (
    <div>
      <h1 className="common__title">Thread list</h1>
      <ThreadIndexTable />
    </div>
  );
}
