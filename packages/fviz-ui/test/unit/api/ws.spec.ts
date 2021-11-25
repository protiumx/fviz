import { WS } from "$/api/ws";
import flushPromises from "flush-promises";

const MockWebSocket = jest.fn();
const getMockClient = () => ({
  addEventListener: jest.fn(),
  send: jest.fn(),
});

global.WebSocket = MockWebSocket as any;

describe("ws", () => {
  test("should create websocket and bind listeners", async () => {
    const client = getMockClient();
    MockWebSocket.mockReturnValueOnce(client);
    const onMessage = (data: any) => {};
    const ws = new WS();
    ws.connect("server", "session", onMessage);

    expect(MockWebSocket).toHaveBeenCalledWith("ws://server/ws/session");

    const onOpen = client.addEventListener.mock.calls[0][1];
    // Resolve promise
    onOpen();
    await flushPromises();

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

  test("should send messages", async () => {
    const client = getMockClient();
    MockWebSocket.mockReturnValueOnce(client);
    const onMessage = (data: any) => {};
    const ws = new WS();
    ws.connect("server", "session", onMessage);

    const onOpen = client.addEventListener.mock.calls[0][1];
    onOpen();
    await flushPromises();

    ws.send({ test: true });
    expect(client.send).toHaveBeenCalledWith(JSON.stringify({ test: true }));
  });

  test("should send raw messages", async () => {
    const client = getMockClient();
    MockWebSocket.mockReturnValueOnce(client);
    const onMessage = (data: any) => {};
    const ws = new WS();
    ws.connect("server", "session", onMessage);

    const onOpen = client.addEventListener.mock.calls[0][1];
    onOpen();
    await flushPromises();

    ws.sendRaw("ping");
    expect(client.send).toHaveBeenCalledWith("ping");
  });

  test("should call onMessage", async () => {
    const client = getMockClient();
    MockWebSocket.mockReturnValueOnce(client);
    const onMessage = jest.fn();
    const ws = new WS();
    ws.connect("server", "session", onMessage);

    const onOpen = client.addEventListener.mock.calls[0][1];
    onOpen();
    await flushPromises();

    // NOTE: it's affected by order of calls
    const onMessageListener = client.addEventListener.mock.calls[3][1];
    onMessageListener({ data: JSON.stringify({ message: "test" }) });

    expect(onMessage).toHaveBeenCalledWith({ message: "test" });
  });
});
