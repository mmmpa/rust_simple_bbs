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

  function submit (e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    createMessage({ threadId, message });
  }

  return (
    <div>
      <h1>new Thread</h1>
      <form onSubmit={submit}>
        <label>first message</label>
        <textarea
          value={message}
          onChange={e => {
            console.log(e.target.value)
            updateMessage(e.target.value)
          }}
        />
        <button type="submit">submit</button>
      </form>
    </div>
  );
});
