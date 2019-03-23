import * as React from 'react';
import { Link } from 'react-router-dom';
import useClock from '../useClock';

export default function GlobalHeader (): JSX.Element {
  const clock = useClock();

  return (
    <div className='global_header'>
      <h1>Simple Thread BBS</h1>
      <div className='gloal_header_navigation'>
        <ul>
          <li>
            <Link to='/'>Home</Link>
          </li>
          <li>
            <Link to='/threads'>Threads</Link>
          </li>
          <li>
            <Link to='/threads/aaa'>Threads aaa</Link>
          </li>
          <li>{clock}</li>
        </ul>
      </div>
    </div>
  );
}
