import * as React from 'react';
import { ThreadItem as T } from '../types';

export default function ThreadItem ({ index, html: __html }: T): JSX.Element {
  return (
    <div className="thread_page__message">
      <h1 className="thread_page__message__index">{index}</h1>
      <div className="thread_page__message__body"dangerouslySetInnerHTML={{ __html }} />
    </div>
  );
}
