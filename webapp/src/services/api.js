import axios, { AxiosInstance } from "axios";

const api = axios.create({
    baseURL: "localhost:8888"
});

export default api;