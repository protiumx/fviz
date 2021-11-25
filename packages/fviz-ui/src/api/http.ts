import api from "$/api/axios";

export interface HandshakeResponse {
  session_uuid: string;
}

export async function postHandshake(
  client: string
): Promise<HandshakeResponse> {
  return (await api.post("/handshake", { client })).data;
}
