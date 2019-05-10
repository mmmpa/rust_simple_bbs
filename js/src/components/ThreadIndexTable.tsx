import React from 'react';
import { Link } from 'react-router-dom';
import { connect } from 'react-redux';
import { AppState, ThreadSummary } from '../types';

const mapStateToProps = (state: AppState) => ({ threadIndex: state.threadIndex });
type Mapped = ReturnType<typeof mapStateToProps>;

export default connect(
  mapStateToProps,
)(function ThreadIndexTable ({ threadIndex }: Mapped): JSX.Element {
  return (
    <div className='thread_list'>
      {threadIndex.summaries.map(t => <Row {...t} key={t.id} />)}
    </div>
  );
});

function Row (props: ThreadSummary): JSX.Element {
  return (
    <Link className='thread_list__item' to={`/threads/${props.id}`}>
      <i className='fa fa-link link--black mr-1 text--small' />
      {props.title}
    </Link>
  );
}
