// src/plugins/api.ts
import axios from "axios";

// Create axios instance
const api = axios.create({
  baseURL:
    (import.meta.env.VITE_API_BASE_URL as string) ||
    "https://eckko.koyeb.app" ||
    "http://192.168.0.170:5006" ||
    "http://10.0.2.2:5006" ||
    "http://localhost:5006",
  headers: {
    "Content-Type": "application/json",
    Accept: "application/json",
  },
  timeout: 10000, 
});

// Request interceptor (e.g., attach auth token automatically)
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem("accessToken"); // or use Pinia store
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// Response interceptor (handle errors globally)
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // example: redirect to login if token expired
      // const router = useRouter() ❌ can't be used here directly
      // → Instead, emit event or handle in composable
      console.warn("Unauthorized, maybe redirect to login");
    }
    return Promise.reject(error);
  }
);

export default api;


// import api from "../plugins/api"; // 👈 use your instance

// const { data, status } = await api.post("/auth/login", { email, password });
