import * as React from 'react';
import { Link } from 'react-router-dom';
import { connect } from 'react-redux';
import { AppState, ThreadSummary } from '../types';

const mapStateToProps = (state: AppState) => ({ threadIndex: state.threadIndex });
type Mapped = ReturnType<typeof mapStateToProps>;

export default connect(
  mapStateToProps,
)(({ threadIndex }: Mapped): JSX.Element => (
  <div className='Box Box--condensed'>
    {threadIndex.summaries.map(t => <Row {...t} />)}
  </div>
));

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
