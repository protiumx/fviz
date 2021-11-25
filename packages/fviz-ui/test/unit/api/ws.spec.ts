import { WS } from "$/api/ws";

const MockWebSocket = jest.fn();
const getMockClient = () => ({
  addEventListener: jest.fn(),
  send: jest.fn(),
});

global.WebSocket = MockWebSocket as any;

describe("ws", () => {
  test("should create websocket and bind listeners", () => {
    const client = getMockClient();
    MockWebSocket.mockReturnValueOnce(client);
    const onMessage = (data: any) => {};
    const _ = new WS("server", onMessage);
    expect(client.addEventListener).toHaveBeenCalledTimes(4);
    expect(client.addEventListener).toHaveBeenCalledWith(
      "open",
      expect.any(Function)
    );
    expect(client.addEventListener).toHaveBeenCalledWith(
      "error",
      expect.any(Function)
    );
    expect(client.addEventListener).toHaveBeenCalledWith(
      "close",
      expect.any(Function)
    );
    expect(client.addEventListener).toHaveBeenCalledWith(
      "message",
      expect.any(Function)
    );
  });

  test("should send messages", () => {
    const client = getMockClient();
    MockWebSocket.mockReturnValueOnce(client);
    const onMessage = (data: any) => {};
    const ws = new WS("server", onMessage);
    ws.send({ test: true });
    expect(client.send).toHaveBeenCalledWith(JSON.stringify({ test: true }));
  });

  test("should call onMessage", () => {
    const client = getMockClient();
    MockWebSocket.mockReturnValueOnce(client);
    const onMessage = jest.fn();
    const ws = new WS("server", onMessage);
    // Get listener handler for messages
    const onMessageListener = client.addEventListener.mock.calls[3][1];
    onMessageListener({ data: JSON.stringify({ message: "test" }) });

    expect(onMessage).toHaveBeenCalledWith({ message: "test" });
  });
});
