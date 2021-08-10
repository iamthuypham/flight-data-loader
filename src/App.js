import './App.css';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useCallback, useState, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

const App = () => {
  const [altitude, setAltitude] = useState(0);

  useEffect(() => {
    listen('rust-event', myCallback)
  }, [])

  const myCallback = useCallback((e) => {
    const batch = JSON.parse(e.payload);
    const num = batch[0].exp_time;
    console.log(e);
    setAltitude(num);
  })

  const handleClick = async () => {
    invoke('my_custom_command').catch(error => console.log("Erorrrrr:", error));
  };

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={handleClick}>Click Me!</button>
        <span>{altitude}</span>
      </header>
    </div>
  );
}

export default App;