export class WS {
  private client: WebSocket | null = null;

  async connect(
    server: string,
    sessionUuid: string,
    onMessage: (data: any) => void
  ): Promise<void> {
    return new Promise((resolve, reject) => {
      this.client = new WebSocket(`ws://${server}/ws/${sessionUuid}`);
      this.client.addEventListener("open", () => {
        resolve();
      });

      this.client.addEventListener("close", () => {
        this.client = null;
        reject();
      });

      this.client.addEventListener("error", () => {
        console.log("[ws] error");
      });

      this.client.addEventListener("message", (e) => {
        onMessage(e.data);
      });
    });
  }

  send(payload: any) {
    this.client!.send(JSON.stringify(payload));
  }

  sendRaw(payload: string) {
    this.client!.send(payload);
  }
}
