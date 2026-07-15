import axios, { type AxiosInstance, type InternalAxiosRequestConfig, type AxiosResponse } from 'axios'
import { useAuthStore } from '@/stores/auth'

// 从 localStorage 读取服务器地址，若无则使用默认值
const getInitialBaseURL = (): string => {
  return localStorage.getItem('apphub_server_url') || 'http://localhost:8080/api/v1'
}

// 创建 axios 实例
const service: AxiosInstance = axios.create({
  baseURL: getInitialBaseURL(),
  timeout: 15000,
})

// 导出设置 baseURL 的函数
export const setBaseURL = (url: string): void => {
  service.defaults.baseURL = url
  localStorage.setItem('apphub_server_url', url)
}

// 请求拦截器：注入 JWT
service.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    const authStore = useAuthStore()
    if (authStore.token) {
      config.headers.Authorization = `Bearer ${authStore.token}`
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 响应拦截器：统一错误处理
service.interceptors.response.use(
  (response: AxiosResponse) => {
    // Blob 响应（文件下载）直接返回，不走 JSON 解析
    if (response.config.responseType === 'blob') {
      return response
    }
    const res = response.data
    // 后端统一返回 { code, message, data } 格式
    if (res.code !== 200) {
      return Promise.reject(new Error(res.message || '请求失败'))
    }
    return res
  },
  (error) => {
    if (error.response) {
      const { status, data } = error.response
      const message = data?.message || `请求失败 (${status})`
      return Promise.reject(new Error(message))
    } else if (error.request) {
      return Promise.reject(new Error('网络错误，请检查后端服务是否启动'))
    }
    return Promise.reject(error)
  }
)

export default service
