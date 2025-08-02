import axios from "axios";

axios.defaults.baseURL =
  (import.meta.env.VITE_API_BASE_URL as string) || "http://localhost:5006";
axios.defaults.headers.common["Content-Type"] = "application/json";
axios.defaults.headers.common["Accept"] = "application/json";
axios.defaults.timeout = 10000; // Set a timeout of 10 seconds

export default axios;
