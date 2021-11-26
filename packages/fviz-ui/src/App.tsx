import React, { useEffect } from "react";

import { WS } from "$/api/ws";
import { postHandshake } from "$/api/http";
import CesiumManager from "$/cesium/manager";
import Drone from "$/devices/drone";

import "./App.scss";

function App() {
  const init = async () => {
    const cesium = new CesiumManager(document.querySelector(".app")!);
    cesium.init();

    const drone = new Drone(cesium.viewer);
    drone.init();
    // NOTE: initial view shows drone from the top
    cesium.viewer.zoomTo(
      drone.entity,
      new Cesium.HeadingPitchRange(0, Cesium.Math.toRadians(-90), 200)
    );
    cesium.viewer.trackedEntity = drone.entity;

    const { session_uuid } = await postHandshake("FVIZ-000");
    const ws = new WS();
    try {
      await ws.connect(
        process.env.REACT_APP_SERVER_HOST as string,
        session_uuid,
        (positionData) => {
          drone.updatePosition(positionData);
        }
      );
    } catch (error) {}
  };

  useEffect(() => {
    init();
  }, []);
  return <main className="app"></main>;
}

export default App;
