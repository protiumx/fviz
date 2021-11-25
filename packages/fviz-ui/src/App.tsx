import React, { useEffect } from "react";

function App() {
  Cesium.Ion.defaultAccessToken = process.env.REACT_APP_CESIUM_TOKEN as string;

  useEffect(() => {
    const viewer = new Cesium.Viewer(document.getElementById("viewer"));

    viewer.entities.add({
      name: "tokyo",
      description: "test",
      position: Cesium.Cartesian3.fromDegrees(139.767052, 35.681167, 100),
      point: { pixelSize: 10 },
    });
  }, []);
  return <div id="viewer"></div>;
}

export default App;
