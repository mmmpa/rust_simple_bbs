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
    <div className="new_thread">
      <h1 className="common__title">Create a new thread</h1>
      <form className="new_thread__form" onSubmit={submit}>
        <div className="new_thread__title">
          <label className="common__label">Title</label>
          <input
            type='text'
            value={title}
            onChange={e => updateThread({ ...threadParams, title: e.target.value })}
          />
        </div>
        <div className="new_thread__message">
          <label className="common__label">First message</label>
          <textarea
            className="new_thread__message__input"
            value={message}
            onChange={e => updateThread({ ...threadParams, message: e.target.value })}
          />
        </div>
        <div className="new_thread__submit">
          <button className="common__submit common--w100" type='submit'>
            <i className="fa fa-plus-circle mr-1" />
            Create !
          </button>
        </div>
      </form>
    </div>
  );
});
