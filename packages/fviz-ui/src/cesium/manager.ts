export default class CesiumManager {
  public viewer!: any;

  constructor(private container: HTMLDivElement) {
    Cesium.Ion.defaultAccessToken = process.env
      .REACT_APP_CESIUM_TOKEN as string;
  }

  init() {
    const viewer = new Cesium.Viewer(this.container, {
      terrainProviderViewModels: [], //Disable terrain changing
      infoBox: false, //Disable InfoBox widget
      selectionIndicator: false, //Disable selection indicator
      navigationInstructionsInitiallyVisible: false,
    });

    this.setScene(viewer);
    this.setClock(viewer);
    //Terrain
    viewer.terrainProvider = Cesium.createWorldTerrain({
      requestWaterMask: true,
      requestVertexNormals: true,
    });
    //Set the random number seed for consistent results.
    Cesium.Math.setRandomNumberSeed(3);
    this.viewer = viewer;
  }

  private setScene({ scene }: any) {
    //Lighting based on sun/moon positions
    scene.globe.enableLighting = false; // generally a bit too dark when enabled
    //Enable depth testing so things behind the terrain disappear.
    scene.globe.depthTestAgainstTerrain = true;
    scene.skyAtmosphere.brightnessShift = 0.2;
  }

  private setClock({ clock }: any) {
    //Make sure viewer is at the desired time.
    const start = Cesium.JulianDate.fromIso8601(new Date().toISOString());
    clock.startTime = start.clone();
    clock.currentTime = start.clone();
    clock.clockRange = Cesium.ClockRange.LOOP_STOP; //Loop at the end
    clock.multiplier = 1;
    clock.shouldAnimate = true; // do not autoplay
  }
}
