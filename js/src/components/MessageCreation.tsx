import { FormEvent } from 'react';
import * as React from 'react';
import { connect } from 'react-redux';
import { useCodemirror } from '../libs/codemirrorHelpers';
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

  const events = { change: e => updateMessage({ message: e.doc.getValue() }) };
  const setView = useCodemirror({ value: message, events, width: '100%', height: 300 });

  return (
    <div className='thread_page__form'>
      <form onSubmit={submit}>
        <div className='message_form__input_area'>
          <label className='common__label'>New message</label>
          <textarea
            className='message_form__message'
            defaultValue={message}
            ref={setView}
          />
        </div>
        <div className='message_form__button_area'>
          <button className='common__submit common--w100' type='submit'>
            <i className='fa fa-plus-circle mr-1' />
            Submit !
          </button>
        </div>
      </form>
    </div>
  );
});
