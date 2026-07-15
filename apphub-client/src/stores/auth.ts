import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import request from '@/utils/request'

// 用户信息类型
interface UserInfo {
  id: number
  username: string
  nickname: string
  email?: string
  avatar?: string
  role: string
}

// 后端统一响应格式
interface ApiResponse<T> {
  code: number
  message: string
  data: T
}

// 登录响应类型
interface LoginResponse {
  access_token: string
  refresh_token: string
  expires_in: number
}

export const useAuthStore = defineStore('auth', () => {
  // State
  const token = ref<string | null>(localStorage.getItem('token'))
  const refreshToken = ref<string | null>(localStorage.getItem('refreshToken'))
  const userInfo = ref<UserInfo | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const isLoggedIn = computed(() => !!token.value)
  const username = computed(() => userInfo.value?.username || '')
  const nickname = computed(() => userInfo.value?.nickname || '')
  const isAdmin = computed(() => userInfo.value?.role === 'admin')

  // Actions
  async function login(username: string, password: string): Promise<boolean> {
    loading.value = true
    error.value = null

    try {
      // 调用后端登录 API
      const response = await request.post<any, ApiResponse<LoginResponse>>('/auth/login', {
        username,
        password,
      })

      const { access_token, refresh_token } = response.data

      // 保存 token
      token.value = access_token
      refreshToken.value = refresh_token
      localStorage.setItem('token', access_token)
      localStorage.setItem('refreshToken', refresh_token)

      // 获取用户信息
      await fetchUserInfo()

      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : '登录失败'
      return false
    } finally {
      loading.value = false
    }
  }

  async function logout(): Promise<void> {
    try {
      // 调用后端登出 API（可选，因为 JWT 是无状态的）
      await request.post('/auth/logout')
    } catch (e) {
      console.error('登出 API 调用失败:', e)
    } finally {
      token.value = null
      refreshToken.value = null
      userInfo.value = null
      localStorage.removeItem('token')
      localStorage.removeItem('refreshToken')
    }
  }

  async function fetchUserInfo(): Promise<void> {
    if (!token.value) return

    try {
      const response = await request.get<any, ApiResponse<UserInfo>>('/auth/user-info')
      userInfo.value = response.data
    } catch (e) {
      console.error('获取用户信息失败:', e)
      // 如果获取失败，清除登录状态
      if (e instanceof Error && e.message.includes('401')) {
        await logout()
      }
    }
  }

  return {
    token,
    refreshToken,
    userInfo,
    loading,
    error,
    isLoggedIn,
    username,
    nickname,
    isAdmin,
    login,
    logout,
    fetchUserInfo,
  }
})
