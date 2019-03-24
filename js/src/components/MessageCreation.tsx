import { FormEvent, useState } from 'react';
import * as React from 'react';
import { connect } from 'react-redux';
import { createMessage } from '../store/actions';

type P = { threadId: string };
const mapDispatchToProps = { createMessage };

type Mapped = P & typeof mapDispatchToProps;

export default connect(
  null,
  mapDispatchToProps,
)(function MessageCreation ({ threadId, createMessage }: Mapped): JSX.Element {
  const [message, setMessage] = useState('');

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
          onChange={e => setMessage(e.target.value)}
        />
        <button type="submit">submit</button>
      </form>
    </div>
  );
});
