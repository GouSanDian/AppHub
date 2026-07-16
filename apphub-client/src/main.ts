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

// 先挂载应用，确保路由正常工作
app.mount('#app')

// 异步加载配置，不阻塞 UI 渲染
const loadConfigAsync = async () => {
  try {
    const configStore = useConfigStore()
    await configStore.load()
    setBaseURL(configStore.serverUrl)

    // 启动时同步服务端地址到 Rust 后端
    if (isTauri) {
      try {
        // TODO
        const { invoke } = await import('@tauri-apps/api/core')
        
        // 添加超时保护，防止 invoke 卡住
        const timeoutPromise = new Promise<never>((_, reject) => {
          setTimeout(() => reject(new Error('同步服务端地址超时')), 3000)
        })
        await Promise.race([
          invoke('sync_server_url', { serverUrl: configStore.serverUrl }),
          timeoutPromise
        ])
        console.log('[启动] 已同步服务端地址到后端:', configStore.serverUrl)
      } catch (e) {
        console.warn('[启动] 同步服务端地址到后端失败:', e)
      }
    }

    // 启动时恢复 token
    const authStore = useAuthStore()
    if (authStore.token && isTauri) {
      try {
        const refreshToken = authStore.refreshToken || ''
        const timeoutPromise = new Promise<never>((_, reject) => {
          setTimeout(() => reject(new Error('恢复 token 超时')), 3000)
        })
        await Promise.race([
          invoke('sync_token', { accessToken: authStore.token, refreshToken }),
          timeoutPromise
        ])
        console.log('[启动] 已恢复 token 到后端')
      } catch (e) {
        console.warn('[启动] 恢复 token 到后端失败:', e)
      }
    }
  } catch (e) {
    console.error('[启动] 加载配置失败:', e)
  }
}

// 延迟执行配置加载，确保路由已完全初始化
setTimeout(() => {
  loadConfigAsync()
}, 100)
