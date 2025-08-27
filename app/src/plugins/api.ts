import axios from "axios";
import { useTokenStore } from "../stores/token";

const api = axios.create({
  baseURL:
    "https://eckko.koyeb.app" ,
    // "http://192.168.0.170:5006",
  headers: {
    "Content-Type": "application/json",
    Accept: "application/json",
  },
  timeout: 10000,
});

api.interceptors.request.use(
  async (config) => {
    const tokenStore = useTokenStore();
    if (!tokenStore.isAccessTokenValid()) {
      await tokenStore.getRefeshToken();
    }

    const token = tokenStore.accessToken;
    config.headers.Authorization = `Bearer ${token}`;
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
