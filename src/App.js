import './App.css';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useCallback, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

const App = () => {
  const [tai, setTai] = useState(0);
  const [truth, setTruth] = useState(null);
  const [dlc, setDlc] = useState(null);

  useEffect(() => {
      setTimeout(() => { closeSplashScreen() }, 1000);
      const taiUnlisten = listen('tai-event', taiCb);
      const truthUnlisten = listen('truth-event', truthCb);
      const dlcUnlisten = listen('dlc-event', dlcCb);

      return () => {
        taiUnlisten.then(f => f());
        truthUnlisten.then(f => f());
        dlcUnlisten.then(f => f());
      }
  }, [])

  const closeSplashScreen = useCallback(async () => {
    return await invoke('close_splashscreen');
  }, [])

  const taiCb = useCallback((e) => {
      const batch = JSON.parse(e.payload);
      setTai(batch)
  })

  const truthCb = useCallback((e) => {
      const batch = JSON.parse(e.payload);
      setTruth(batch[0])
  })

  const dlcCb = useCallback((e) => {
    const batch = JSON.parse(e.payload);
    setDlc(batch[0])
  })

  const handleClick = async () => {
    invoke('my_custom_command').catch(error => console.log("Erorrrrr:", error));
  };

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={handleClick}>Click Me!</button>
        {<table>
          <tr>
          <td>Time: </td>
          <td>{tai}</td>
          </tr>
        </table>}
        {<table>
            <tr>
              <td>truth_pos_CON_ECEF_ECEF_M[1]</td>
              <td>{truth && truth['truth_pos_CON_ECEF_ECEF_M[1]']}</td>
            </tr>
            <tr>
              <td>truth_pos_CON_ECEF_ECEF_M[2]</td>
              <td>{truth && truth['truth_pos_CON_ECEF_ECEF_M[2]']}</td>
            </tr>
            <tr>
              <td>truth_pos_CON_ECEF_ECEF_M[3]</td>
              <td>{truth && truth['truth_pos_CON_ECEF_ECEF_M[3]']}</td>
            </tr>
            <tr>
              <td>truth_vel_CON_ECEF_ECEF_MpS[1]</td>
              <td>{truth && truth['truth_vel_CON_ECEF_ECEF_MpS[1]']}</td>
            </tr>
            <tr>
              <td>truth_vel_CON_ECEF_ECEF_MpS[2]</td>
              <td> {truth && truth['truth_vel_CON_ECEF_ECEF_MpS[2]']}</td>
            </tr>
            <tr>
              <td>truth_vel_CON_ECEF_ECEF_MpS[3]</td>
              <td> {truth && truth['truth_vel_CON_ECEF_ECEF_MpS[3]']}</td>
            </tr>
            <tr>
              <td>truth_quat_CON2ECEF[1]</td>
              <td> {truth && truth['truth_quat_CON2ECEF[1]']}</td>
            </tr>
            <tr>
              <td>truth_quat_CON2ECEF[2]</td>
              <td> {truth && truth['truth_quat_CON2ECEF[2]']}</td>
            </tr>
            <tr>
              <td>truth_quat_CON2ECEF[3]</td>
              <td> {truth && truth['truth_quat_CON2ECEF[3]']}</td></tr>
            <tr>
              <td>truth_quat_CON2ECEF[4]</td>
              <td> {truth && truth['truth_quat_CON2ECEF[4]']}</td>
            </tr>
          </table>
        }
        {<table>
            <tr>
              <td>DATA_DELTA_VEL[1]</td>
              <td> {dlc && dlc['DATA_DELTA_VEL[1]']}</td>
            </tr>
            <tr>
              <td>DATA_DELTA_VEL[2]</td>
              <td> {dlc && dlc['DATA_DELTA_VEL[2]']}</td>
            </tr>
            <tr>
              <td>DATA_DELTA_VEL[3]</td>
              <td> {dlc && dlc['DATA_DELTA_VEL[3]']}</td>
            </tr>
            <tr>
              <td>DATA_DELTA_ANGLE[1]</td>
              <td> {dlc && dlc['DATA_DELTA_ANGLE[1]']}</td>
            </tr>
            <tr>
              <td>DATA_DELTA_ANGLE[2]</td>
              <td> {dlc && dlc['DATA_DELTA_ANGLE[2]']}</td>
            </tr>
            <tr>
              <td>DATA_DELTA_ANGLE[3]</td>
              <td> {dlc && dlc['DATA_DELTA_ANGLE[3]']}</td>
            </tr>
          </table>
        }
      </header>
    </div>
  );
}

export default App;