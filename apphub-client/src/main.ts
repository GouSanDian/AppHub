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

const app = createApp(App)

// 注册Element Plus图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

const pinia = createPinia()
app.use(pinia)

// 加载配置并设置 baseURL
const configStore = useConfigStore()
configStore.load().then(() => {
  setBaseURL(configStore.serverUrl)
})

app.use(router)
app.use(ElementPlus)

app.mount('#app')
