import * as React from 'react';
import { Link } from 'react-router-dom';

export default function GlobalFooter (): JSX.Element {
  return (
    <div className='global_footer'>
      <div className='gloal_footer_navigation'>
        <ul>
          <li>
            <Link to='/'>Home</Link>
          </li>
          <li>
            <Link to='/about'>About</Link>
          </li>
          <li>
            <Link to='/topics/eee'>Topics</Link>
          </li>
        </ul>
      </div>
    </div>
  );
}
