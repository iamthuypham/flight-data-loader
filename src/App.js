import './App.css';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useCallback, useState, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

const App = () => {
  const [isInitialLoad, setIsInitialLoad] = useState(true);
  const [tai, setTai] = useState(null);
  const [truth, setTruth] = useState(null);
  const [lidar, setLidar] = useState(null);

  useEffect(() => {
    if (isInitialLoad) {
      setTimeout(() => { closeSplashScreen() }, 5000);
      // listen('tai-event', taiCb);
      listen('truth-event', truthCb);
      // listen('lidar-event', lidarCb);
      setIsInitialLoad(false);
    }
  }, [isInitialLoad])

  const closeSplashScreen = useCallback(async () => {
    return await invoke('close_splashscreen');
  }, [])

  // const taiCb = useCallback((e) => {
  //   const batch = JSON.parse(e.payload);
  //   setTai(batch)
  // })

  const truthCb = useCallback((e) => {
    const batch = JSON.parse(e.payload);
    setTruth(batch[0])
  })

  // const lidarCb = useCallback((e) => {
  //   const batch = JSON.parse(e.payload);
  //   setLidar(batch[0])
  // })

  const handleClick = async () => {
    invoke('my_custom_command').catch(error => console.log("Erorrrrr:", error));
  };

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={handleClick}>Click Me!</button>
        {tai && <div>
          <span>Time</span>
          <span>{tai}</span>
        </div>}
        {truth && <table>
            <tr>
              <td>truth_pos_CON_ECEF_ECEF_M[1]</td>
              <td>{truth['truth_pos_CON_ECEF_ECEF_M[1]']}</td>
            </tr>
            <tr>
              <td>truth_pos_CON_ECEF_ECEF_M[2]</td>
              <td>{truth['truth_pos_CON_ECEF_ECEF_M[2]']}</td>
            </tr>
            <tr>
              <td>truth_pos_CON_ECEF_ECEF_M[3]</td>
              <td>{truth['truth_pos_CON_ECEF_ECEF_M[3]']}</td>
            </tr>
            <tr>
              <td>truth_vel_CON_ECEF_ECEF_MpS[1]</td>
              <td>{truth['truth_vel_CON_ECEF_ECEF_MpS[1]']}</td>
            </tr>
            <tr>
              <td>truth_vel_CON_ECEF_ECEF_MpS[2]</td>
              <td> {truth['truth_vel_CON_ECEF_ECEF_MpS[2]']}</td>
            </tr>
            <tr>
              <td>truth_vel_CON_ECEF_ECEF_MpS[3]</td>
              <td> {truth['truth_vel_CON_ECEF_ECEF_MpS[3]']}</td>
            </tr>
            <tr>
              <td>truth_quat_CON2ECEF[1]</td>
              <td> {truth['truth_quat_CON2ECEF[1]']}</td>
            </tr>
            <tr>
              <td>truth_quat_CON2ECEF[2]</td>
              <td> {truth['truth_quat_CON2ECEF[2]']}</td>
            </tr>
            <tr>
              <td>truth_quat_CON2ECEF[3]</td>
              <td> {truth['truth_quat_CON2ECEF[3]']}</td></tr>
            <tr>
              <td>truth_quat_CON2ECEF[4]</td>
              <td> {truth['truth_quat_CON2ECEF[4]']}</td>
            </tr>
          </table>
        }
        {lidar && <table>
            <tr>
              <td>OMPS_Range_M[1]</td>
              <td> {truth['OMPS_Range_M[1]']}</td>
            </tr>
            <tr>
              <td>OMPS_Range_M[2]</td>
              <td> {truth['OMPS_Range_M[2]']}</td>
            </tr>
            <tr>
              <td>OMPS_Range_M[3]</td>
              <td> {truth['OMPS_Range_M[3]']}</td>
            </tr>
            <tr>
              <td>OMPS_Range_M[4]</td>
              <td> {truth['OMPS_Range_M[4]']}</td>
            </tr>
            <tr>
              <td>OMPS_DopplerSpeed_MpS[1]</td>
              <td> {truth['OMPS_DopplerSpeed_MpS[1]']}</td>
            </tr>
            <tr>
              <td>OMPS_DopplerSpeed_MpS[2]</td>
              <td> {truth['OMPS_DopplerSpeed_MpS[2]']}</td>
            </tr>
            <tr>
              <td>OMPS_DopplerSpeed_MpS[3]</td>
              <td> {truth['OMPS_DopplerSpeed_MpS[3]']}</td>
            </tr>
            <tr>
              <td>OMPS_DopplerSpeed_MpS[4]</td>
              <td> {truth['OMPS_DopplerSpeed_MpS[4]']}</td>
            </tr>
          </table>
        }
      </header>
    </div>
  );
}

export default App;