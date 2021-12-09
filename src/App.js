import './App.css';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useCallback, useState, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

const App = () => {
  const [isInitialLoad, setIsInitialLoad] = useState(true);
  const [value, setValue] = useState(null);

  useEffect(() => {
    if (isInitialLoad) {
      setTimeout(() => { closeSplashScreen() }, 4000);
      listen('rust-event', myCallback);
      setIsInitialLoad(false);
    }
  }, [isInitialLoad])

  const closeSplashScreen = useCallback(async () => {
    return await invoke('close_splashscreen');
  }, [])

  const myCallback = useCallback((e) => {
    const batch = JSON.parse(e.payload);
    setValue(batch[0])
  })

  const handleClick = async () => {
    invoke('my_custom_command').catch(error => console.log("Erorrrrr:", error));
  };

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={handleClick}>Click Me!</button>
        {value && <table>
            <tr>
              <td>Time</td>
              <td>{value.TIME_NANOSECONDS_TAI}</td>
            </tr>
            <tr>
              <td>truth_pos_CON_ECEF_ECEF_M[1]</td>
              <td>{value['truth_pos_CON_ECEF_ECEF_M[1]']}</td>
            </tr>
            <tr>
              <td>truth_pos_CON_ECEF_ECEF_M[2]</td>
              <td>{value['truth_pos_CON_ECEF_ECEF_M[2]']}</td>
            </tr>
            <tr>
              <td>truth_pos_CON_ECEF_ECEF_M[3]</td>
              <td>{value['truth_pos_CON_ECEF_ECEF_M[3]']}</td>
            </tr>
            <tr>
              <td>truth_vel_CON_ECEF_ECEF_MpS[1]</td>
              <td>{value['truth_vel_CON_ECEF_ECEF_MpS[1]']}</td>
            </tr>
            <tr>
              <td>truth_vel_CON_ECEF_ECEF_MpS[2]</td>
              <td> {value['truth_vel_CON_ECEF_ECEF_MpS[2]']}</td>
            </tr>
            <tr>
              <td>truth_vel_CON_ECEF_ECEF_MpS[3]</td>
              <td> {value['truth_vel_CON_ECEF_ECEF_MpS[3]']}</td>
            </tr>
            <tr>
              <td>truth_quat_CON2ECEF[1]</td>
              <td> {value['truth_quat_CON2ECEF[1]']}</td>
            </tr>
            <tr>
              <td>truth_quat_CON2ECEF[2]</td>
              <td> {value['truth_quat_CON2ECEF[2]']}</td>
            </tr>
            <tr>
              <td>truth_quat_CON2ECEF[3]</td>
              <td> {value['truth_quat_CON2ECEF[3]']}</td></tr>
            <tr>
              <td>truth_quat_CON2ECEF[4]</td>
              <td> {value['truth_quat_CON2ECEF[4]']}</td>
            </tr>
          </table>
        }
      </header>
    </div>
  );
}

export default App;