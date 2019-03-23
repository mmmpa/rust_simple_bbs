import * as React from 'react';
import { Link } from 'react-router-dom';
import { connect } from 'react-redux';
import { AppState, ThreadSummary } from '../types';

const mapStateToProps = (state: AppState) => ({ threadIndex: state.threadIndex });
type Mapped = ReturnType<typeof mapStateToProps>;

export default connect(
  mapStateToProps,
)(function ThreadIndexTable ({ threadIndex }: Mapped): JSX.Element {
  return (
    <div className='Box Box--condensed'>
      <h1>Table</h1>
      {threadIndex.summaries.map(t => <Row {...t} key={t.id} />)}
    </div>
  );
});

function Row (props: ThreadSummary): JSX.Element {
  return (
    <Link className='Box-row block' to={`/threads/${props.id}`}>
      <div className='TableObject-item TableObject-item--primary'>
        {props.title}
      </div>
      <div className='TableObject-item'>
        {props.count}
      </div>
    </Link>
  );
}
