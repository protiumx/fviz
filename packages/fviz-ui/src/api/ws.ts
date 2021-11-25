export class WS {
  private client: WebSocket | null = null;

  constructor(server: string, onMessage: (msg: any) => void) {
    this.client = new WebSocket(`ws://${server}`);
    this.client.addEventListener("open", () => {
      console.log("[ws] connected");
    });

    this.client.addEventListener("close", () => {
      console.log("[ws] disconnected");
      this.client = null;
    });

    this.client.addEventListener("error", () => {
      console.log("[ws] error");
    });

    this.client.addEventListener("message", (e) => {
      onMessage(JSON.parse(e.data));
    });
  }

  send(payload: any) {
    this.client!.send(JSON.stringify(payload));
  }
}
