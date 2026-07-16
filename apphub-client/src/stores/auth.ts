import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import request from '@/utils/request'
// import { invoke } from '@tauri-apps/api/core'

const isTauri = typeof window !== 'undefined' && '__TAURI__' in window

// 用户信息类型
interface UserInfo {
  id: number
  username: string
  nickname: string
  email?: string
  avatar?: string
  role: string       // 'super_admin' | 'admin' | 'user'
  role_id?: number
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
  const role = computed(() => userInfo.value?.role || '')
  const roleId = computed(() => userInfo.value?.role_id ?? 0)
  const isSuperAdmin = computed(() => userInfo.value?.role === 'super_admin')
  // 管理员（包含超级管理员和普通管理员）
  const isAdmin = computed(() =>
    userInfo.value?.role === 'admin' || userInfo.value?.role === 'super_admin'
  )
  // 判断当前用户是否拥有指定角色（支持传入角色名或角色 id）
  function hasRole(target: string | number): boolean {
    if (typeof target === 'number') return roleId.value === target
    return role.value === target
  }
  // 判断当前用户是否拥有管理员或超级管理员权限
  function canManageUsers(): boolean {
    return isAdmin.value
  }

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

      // 同步 token 和 server URL 到 Rust 后端，供扫描服务等使用
      if (isTauri) {
        try {
          // TODO
          const { invoke } = await import('@tauri-apps/api/core')
          await invoke('sync_token', { accessToken: access_token, refreshToken: refresh_token })
          // 同步服务端地址
          const serverUrl = localStorage.getItem('apphub_server_url') || 'http://localhost:8080/api/v1'
          await invoke('sync_server_url', { serverUrl })
        } catch (e) {
          console.error('同步 token 到后端失败:', e)
        }
      }

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

      // 清除 Rust 后端的 token
      if (isTauri) {
        try {
          await invoke('logout')
        } catch (e) {
          console.error('清除后端 token 失败:', e)
        }
      }
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
    role,
    roleId,
    isAdmin,
    isSuperAdmin,
    hasRole,
    canManageUsers,
    login,
    logout,
    fetchUserInfo,
  }
})
