import axios from "axios";

const api = axios.create({
  baseURL: `http://${process.env.REACT_APP_SERVER_HOST}`,
  headers: {
    "Content-Type": "application/json",
    Accept: "application/json",
  },
});

export default api;
