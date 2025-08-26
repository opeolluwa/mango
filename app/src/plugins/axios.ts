import axios from "axios";

// axios.defaults.baseURL =
//   (import.meta.env.VITE_API_BASE_URL as string) ||
//   "https://eckko.koyeb.app/" ||
//   "http://192.168.0.170:5006" ||
//   "http://10.0.2.2:5006" ||
//   "http://localhost:5006";

axios.defaults.baseURL = "http://192.168.0.170:5006";
// axios.defaults.baseURL = "https://eckko.koyeb.app";
axios.defaults.headers.common["Content-Type"] = "application/json";
axios.defaults.headers.common["Accept"] = "application/json";
// axios.defaults.timeout = 10000; // Set a timeout of 10 seconds

export default axios;
