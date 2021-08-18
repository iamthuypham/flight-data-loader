import './App.css';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useCallback, useState, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

const App = () => {
  const [value, setValue] = useState(null);

  useEffect(() => {
    listen('rust-event', myCallback)
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
        {value &&
        <>
        <span>Exp Time: {value.exp_time}</span>
        <span>Utc Time: {value.utc_time}</span>
        <span>Lat Long 1: {value.lat_long_1}</span>
        <span>Lat Long 2:{value.lat_long_2}</span>
        <span>Pos 1: {value.pos_1}</span>
        <span>Pos 2: {value.pos_2}</span>
        <span>Pos 3: {value.pos_3}</span>
        <span>Gps alt: {value.gps_alt}</span>
        <span>Vel 1: {value.vel_1}</span>
        <span>Vel 2: {value.vel_2}</span>
        <span>Vel 3:{value.vel_3}</span>
        <span>Accel 1: {value.accel_1}</span>
        <span>Accel 2: {value.accel_2}</span>
        <span>Accel 3: {value.accel_3}</span>
        <span>Mag Accel: {value.mag_accel}</span>
        <span>Att 1: {value.att_1}</span>
        <span>Att 2: {value.att_2}</span>
        <span>Att 3: {value.att_3}</span>
        <span>Ang vel: {value.ang_vel_1}</span>
        <span>Ang vel: {value.ang_vel_2}</span>
        <span>Ang vel: {value.ang_vel_3}</span>
        <span>Warning Liff Off: {value.warnings_liftoff_warn}</span>
        <span>Warning Rcs: {value.warnings_rcs_warn}</span>
        <span>Warning Drogue: {value.warnings_drogue_chute_warn}</span>
        <span>Warning Landing: {value.warnings_landing_warn}</span>
        <span>Warning Chute: {value.warnings_chute_fault_warn}</span>
        </>
        }
      </header>
    </div>
  );
}

export default App;