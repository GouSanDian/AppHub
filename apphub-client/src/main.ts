import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'

import App from './App.vue'
import router from './router'
import './styles/main.css'
import { useConfigStore } from '@/stores/config'
import { setBaseURL } from '@/utils/request'
import { useAuthStore } from '@/stores/auth'
// import { invoke } from '@tauri-apps/api/core'

const app = createApp(App)

// 注册Element Plus图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

const pinia = createPinia()
app.use(pinia)
app.use(router)
app.use(ElementPlus)

const isTauri = typeof window !== 'undefined' && '__TAURI__' in window

// ✅ 修复：在挂载前同步/异步完成关键配置初始化，避免竞态条件
const initApp = async () => {
  try {
    const configStore = useConfigStore()
    await configStore.load()
    setBaseURL(configStore.serverUrl)

    // 启动时同步服务端地址到 Rust 后端
    if (isTauri) {
      try {
        await invoke('sync_server_url', { serverUrl: configStore.serverUrl })
        console.log('[启动] 已同步服务端地址到后端:', configStore.serverUrl)
      } catch (e) {
        console.warn('[启动] 同步服务端地址到后端失败:', e)
      }
    }

    // 启动时恢复 token 到后端
    const authStore = useAuthStore()
    if (authStore.token && isTauri) {
      try {
        const refreshToken = authStore.refreshToken || ''
        await invoke('sync_token', { accessToken: authStore.token, refreshToken })
        console.log('[启动] 已恢复 token 到后端')
      } catch (e) {
        console.warn('[启动] 恢复 token 到后端失败:', e)
      }
    }
  } catch (e) {
    console.error('[启动] 初始化配置失败:', e)
  }
  
  // ✅ 确保关键初始化完成后再挂载应用
  app.mount('#app')
}

initApp()