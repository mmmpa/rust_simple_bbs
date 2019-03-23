import * as React from 'react';
import { connect } from 'react-redux';
import { AppState } from '../types';
import MessageCreation from './MessageCreation';
import ThreadItem from './ThreadItem';

const mapStateToProps = (state: AppState) => ({ threadBody: state.threadBody });
type Mapped = ReturnType<typeof mapStateToProps>;

function itemsElement (items) {
  return items.map(item => <ThreadItem {...item} key={items.index} />);
}

export default connect(
  mapStateToProps,
)(function ThreadBody ({ threadBody: { title, messages, id } }: Mapped): JSX.Element {
  return (
    <div>
      <h1>Thread: {title}</h1>
      {itemsElement(messages)}
      <MessageCreation />
    </div>
  );
});
