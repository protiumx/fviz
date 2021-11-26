function computePositionProperty(
  altitudeOffset: number,
  positionData: number[]
) {
  var property = new Cesium.SampledPositionProperty();
  // Simulate realtime data
  const time = Cesium.JulianDate.fromDate(new Date());
  const position = Cesium.Cartesian3.fromDegrees(
    positionData[0],
    positionData[1],
    positionData[2] + altitudeOffset
  );
  property.addSample(time, position);
  return property;
}

export default class Drone {
  public entity: any = null;
  constructor(private viewer: any) {}

  init() {
    // Actually create the entity
    this.entity = this.viewer.entities.add({
      //Load the Cesium plane model to represent the entity
      model: {
        uri: "models/iris.glb",
        minimumPixelSize: 64,
        scale: 20,
      },

      position: Cesium.Cartesian3.fromDegrees(-117.0763111, 32.387429, 20.952),

      //Show the path as a yellow line
      path: {
        resolution: 1,
        material: new Cesium.PolylineGlowMaterialProperty({
          glowPower: 0.1,
          color: Cesium.Color.YELLOW,
        }),
        width: 10,
      },
    });
  }

  updatePosition(positionData: string) {
    const [lng, lat, alt] = positionData.split(" ").map(Number.parseFloat);
    this.entity.position = Cesium.Cartesian3.fromDegrees(lng, lat, alt);
  }
}
