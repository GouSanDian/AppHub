import { defineStore } from 'pinia'
import { ref } from 'vue'
// import { invoke } from '@tauri-apps/api/core'

const isTauri = typeof window !== 'undefined' && '__TAURI__' in window

const DEFAULT_SERVER_URL = 'http://localhost:8080/api/v1'

interface TauriAppConfig {
  server_url: string
  download_path: string
  auto_start: boolean
  minimize_to_tray: boolean
  scan_enabled: boolean
  scan_interval: number
}

function getDefaultDownloadPath(): string {
  // 浏览器环境 fallback
  return `${window.navigator.userAgent.includes('Win') ? 'C:\\Users\\User' : '~'}/Downloads`
}

export const useConfigStore = defineStore('config', () => {
  const serverUrl = ref(DEFAULT_SERVER_URL)
  const downloadPath = ref('')
  const autoStart = ref(false)
  const minimizeToTray = ref(true)
  const scanEnabled = ref(true)
  const scanInterval = ref(300)

  async function load() {
    if (isTauri) {
      try {
        const { invoke } = await import('@tauri-apps/api/core')
        const cfg = await invoke<TauriAppConfig>('get_config')
        serverUrl.value = cfg.server_url || DEFAULT_SERVER_URL
        downloadPath.value = cfg.download_path || getDefaultDownloadPath()
        autoStart.value = cfg.auto_start
        minimizeToTray.value = cfg.minimize_to_tray
        scanEnabled.value = cfg.scan_enabled
        scanInterval.value = cfg.scan_interval
      } catch (e) {
        console.error('加载配置失败:', e)
      }
    } else {
      try {
        const saved = localStorage.getItem('apphub_config')
        if (saved) {
          const cfg = JSON.parse(saved)
          serverUrl.value = cfg.serverUrl || DEFAULT_SERVER_URL
          downloadPath.value = cfg.downloadPath || getDefaultDownloadPath()
          autoStart.value = cfg.autoStart ?? false
          minimizeToTray.value = cfg.minimizeToTray ?? true
          scanEnabled.value = cfg.scanEnabled ?? true
          scanInterval.value = cfg.scanInterval ?? 300
        } else {
          downloadPath.value = getDefaultDownloadPath()
        }
      } catch {
        downloadPath.value = getDefaultDownloadPath()
      }
    }
    // 同步 serverUrl 到 localStorage，供 request.ts 初始读取
    localStorage.setItem('apphub_server_url', serverUrl.value)
  }

  async function save() {
    if (isTauri) {
      try {
        const { invoke } = await import('@tauri-apps/api/core')
        await invoke('set_config', { key: 'server_url', value: serverUrl.value })
        await invoke('set_config', { key: 'download_path', value: downloadPath.value })
        await invoke('set_config', { key: 'auto_start', value: autoStart.value })
        await invoke('set_config', { key: 'minimize_to_tray', value: minimizeToTray.value })
        await invoke('set_config', { key: 'scan_enabled', value: scanEnabled.value })
        await invoke('set_config', { key: 'scan_interval', value: scanInterval.value })
        await invoke('save_config')
      } catch (e) {
        console.error('保存配置失败:', e)
        throw e
      }
    } else {
      localStorage.setItem(
        'apphub_config',
        JSON.stringify({
          serverUrl: serverUrl.value,
          downloadPath: downloadPath.value,
          autoStart: autoStart.value,
          minimizeToTray: minimizeToTray.value,
          scanEnabled: scanEnabled.value,
          scanInterval: scanInterval.value,
        })
      )
    }
    localStorage.setItem('apphub_server_url', serverUrl.value)
  }

  function resetToDefaults() {
    serverUrl.value = DEFAULT_SERVER_URL
    downloadPath.value = getDefaultDownloadPath()
    autoStart.value = false
    minimizeToTray.value = true
    scanEnabled.value = true
    scanInterval.value = 300
  }

  return {
    serverUrl,
    downloadPath,
    autoStart,
    minimizeToTray,
    scanEnabled,
    scanInterval,
    load,
    save,
    resetToDefaults,
  }
})
