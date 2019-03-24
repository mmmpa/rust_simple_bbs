import { FormEvent } from 'react';
import * as React from 'react';
import { connect } from 'react-redux';
import { createThread, updateThread } from '../store/actions';
import { AppState } from '../types';


const mapDispatchToProps = { createThread, updateThread };
const mapStateToProps = (state: AppState) => ({ threadParams: state.threadParams });
type Mapped = ReturnType<typeof mapStateToProps> & typeof mapDispatchToProps;

export default connect(
  mapStateToProps,
  mapDispatchToProps,
)(function ThreadCreation ({ createThread, threadParams, updateThread }: Mapped): JSX.Element {
  const { title, message } = threadParams;

  function submit (e: FormEvent<HTMLFormElement>) {
    e.preventDefault();

    createThread();
  }

  return (
    <div>
      <h1>new Thread</h1>
      <form onSubmit={submit}>
        <label>title</label>
        <input
          type='text'
          value={title}
          onChange={e => updateThread({ ...threadParams, title: e.target.value })}
        />
        <label>first message</label>
        <textarea
          value={message}
          onChange={e => updateThread({ ...threadParams, message: e.target.value })}
        />
        <button type='submit'>submit</button>
      </form>
    </div>
  );
});
