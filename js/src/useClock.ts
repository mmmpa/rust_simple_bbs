import { useEffect, useState } from 'react';

export default function useClock (): string {
  const [clock, setClock] = useState(new Date().toISOString());

  useEffect(() => {
    const id = setInterval(() => setClock(new Date().toISOString()), 1000);
    return () => clearInterval(id);
  });

  return clock;
}
