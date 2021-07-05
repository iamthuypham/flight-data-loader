import './App.css';
import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react';

const App = () => {
  const [data, setData] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    if (!data) {
      invoke('my_custom_command').then((message) => setData(message)).catch(error => setError(error))
    }
    console.log(data)
  }, [data])
  console.log("Any Error?:", error)
  return (
    <div className="App">
      <header className="App-header">
        <p>{data || "nothing yet"}</p>
      </header>
    </div>
  );
}

export default App;
