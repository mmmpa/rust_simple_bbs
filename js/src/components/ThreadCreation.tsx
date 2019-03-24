import { FormEvent, useState } from 'react';
import * as React from 'react';
import { connect } from 'react-redux';
import { createThread } from '../store/actions';


const mapDispatchToProps = { createThread };

type Mapped = typeof mapDispatchToProps;

export default connect(
  null,
  mapDispatchToProps,
)(function ThreadCreation ({ createThread }: Mapped): JSX.Element {
  const [title, setTitle] = useState('');
  const [message, setMessage] = useState('');

  function submit (e: FormEvent<HTMLFormElement>) {
    e.preventDefault();

    createThread({ title, message });
  }

  return (
    <div>
      <h1>new Thread</h1>
      <form onSubmit={submit}>
        <label>title</label>
        <input
          type='text'
          value={title}
          onChange={e => setTitle(e.target.value)}
        />
        <label>first message</label>
        <textarea
          value={message}
          onChange={e => setMessage(e.target.value)}
        />
        <button type='submit'>submit</button>
      </form>
    </div>
  );
});
