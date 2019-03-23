import * as React from 'react';
import { ThreadItem as T } from '../types';

export default function ThreadItem ({ index, html: __html }: T): JSX.Element {
  return (
    <>
      <h1>{index}</h1>
      <div dangerouslySetInnerHTML={{ __html }} />
    </>
  );
}
