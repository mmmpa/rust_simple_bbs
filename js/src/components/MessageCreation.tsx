import { FormEvent } from 'react';
import * as React from 'react';
import { connect } from 'react-redux';
import { createMessage, updateMessage } from '../store/actions';
import { AppState } from '../types';

type P = { threadId: string };
const mapStateToProps = (state: AppState) => ({ messageParams: state.messageParams });
const mapDispatchToProps = { createMessage, updateMessage };

type Mapped = P & ReturnType<typeof mapStateToProps> & typeof mapDispatchToProps;

export default connect(
  mapStateToProps,
  mapDispatchToProps,
)(function MessageCreation ({ threadId, createMessage, updateMessage, messageParams }: Mapped): JSX.Element {
  const { message } = messageParams;

  function submit (e: FormEvent) {
    e.preventDefault();
    createMessage({ threadId });
  }

  return (
    <div className="message_form">
      <form onSubmit={submit}>
        <div className="message_form__input_area">
          <label className="message_form__label">message</label>
          <textarea
            className="message_form__message"
            value={message}
            onChange={e => updateMessage({ message: e.target.value })}
          />
        </div>
        <div className="message_form__button_area">
          <button
            className="message_form__submit"
            type='submit'>submit
          </button>
        </div>
      </form>
    </div>
  );
});
