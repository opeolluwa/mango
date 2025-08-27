import axios from "axios";


axios.defaults.baseURL = "http://192.168.0.170:5006";
// axios.defaults.baseURL = "https://eckko.koyeb.app";
axios.defaults.headers.common["Content-Type"] = "application/json";
axios.defaults.headers.common["Accept"] = "application/json";
// axios.defaults.timeout = 10000; // Set a timeout of 10 seconds

export default axios;


