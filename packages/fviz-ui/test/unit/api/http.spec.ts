import api from "$/api/axios";
import { postHandshake } from "$/api/http";

jest.mock("$/api/axios");

describe("http", () => {
  test("should post handshake", async () => {
    (api.post as jest.Mock).mockResolvedValueOnce({
      data: { session_uuid: "session" },
    });
    const response = await postHandshake("client");
    expect(response).toEqual({
      session_uuid: "session",
    });
    expect(api.post).toHaveBeenCalledWith("/handshake", {
      client: "client",
    });
  });
});
