import React from 'react';
import { ThreadItem as T } from '../types';

export default function ThreadItem ({ index, html: __html }: T): JSX.Element {
  return (
    <div className='thread_page__message' id={`${index}`}>
      <h1 className='thread_page__message__index'>
        <div className='thread_page__message__index__inner'>
          {index}
        </div>
      </h1>
      <div className='thread_page__message__body '>
        <div className='markdown-body' dangerouslySetInnerHTML={{ __html }} />
      </div>
    </div>
  );
}
