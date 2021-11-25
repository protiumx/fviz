import axios from "axios";

const api = axios.create({
  baseURL: `http://${process.env.REACT_APP_SERVER_URL}`,
  headers: {
    "Content-Type": "application/json",
    Accept: "application/json",
  },
});

export default api;
