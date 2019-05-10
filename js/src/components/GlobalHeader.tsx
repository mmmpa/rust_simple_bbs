import React from 'react';
import { Link } from 'react-router-dom';
import useClock from '../useClock';

export default function GlobalHeader (): JSX.Element {
  const clock = useClock();

  return (
    <div className='global_header'>
      <h1 className='global_header__title'>STBBS</h1>
      <ul className='global_header__navigation'>
        <li className='global_header__navigation__item'>
          <Link className='link--white' to='/'>
            <i className='fa fa-list mr-1' />
            Thread list
          </Link>
        </li>
        <li className='global_header__navigation__item'>
          <Link className='link--white' to='/new_thread'>
            <i className='fa fa-plus-circle mr-1' />
            Create a new thread
          </Link>
        </li>
        <li className='global_header__navigation__item--clock'>{clock}</li>
      </ul>
    </div>
  );
}
