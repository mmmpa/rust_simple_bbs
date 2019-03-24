import * as React from 'react';
import { Link } from 'react-router-dom';
import useClock from '../useClock';

export default function GlobalHeader (): JSX.Element {
  const clock = useClock();

  return (
    <div className='global_header'>
      <h1>STBBS</h1>
      <div className='gloal_header_navigation'>
        <ul>
          <li>
            <Link className="link--white" to='/'>Thread list</Link>
          </li>
          <li>
            <Link className="link--white" to='/new_thread'>Create a new thread</Link>
          </li>
          <li>{clock}</li>
        </ul>
      </div>
    </div>
  );
}
