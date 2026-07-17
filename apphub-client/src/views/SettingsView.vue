<template>
  <div class="settings">
    <h2 class="page-title">设置</h2>

    <el-row :gutter="20">
      <el-col :xs="24" :lg="12">
        <el-card class="setting-card">
          <template #header>
            <div class="card-header">
              <el-icon><Setting /></el-icon>
              <span>基本设置</span>
            </div>
          </template>

          <el-form label-position="top">
            <el-form-item label="下载保存路径">
              <el-input v-model="config.downloadPath" readonly>
                <template #append>
                  <el-button :icon="FolderOpened" @click="selectDownloadPath" />
                </template>
              </el-input>
            </el-form-item>

            <el-form-item label="开机自启动">
              <el-switch v-model="config.autoStart" />
            </el-form-item>

            <el-form-item label="最小化到系统托盘">
              <el-switch v-model="config.minimizeToTray" />
            </el-form-item>
          </el-form>
        </el-card>
      </el-col>

      <el-col :xs="24" :lg="12">
        <el-card class="setting-card">
          <template #header>
            <div class="card-header">
              <el-icon><Monitor /></el-icon>
              <span>安全监控</span>
            </div>
          </template>

          <el-form label-position="top">
            <el-form-item label="启用进程监控">
              <el-switch v-model="config.scanEnabled" />
            </el-form-item>

            <el-form-item label="扫描间隔（分钟）">
              <el-slider
                v-model="config.scanInterval"
                :min="1"
                :max="60"
                :step="1"
                show-stops
                show-input
              />
            </el-form-item>

            <el-form-item>
              <el-alert
                title="进程监控需要管理员权限"
                type="warning"
                :closable="false"
                show-icon
              />
            </el-form-item>
          </el-form>
        </el-card>
      </el-col>
    </el-row>



    <div class="actions">
      <el-button type="primary" size="large" @click="saveSettings">保存设置</el-button>
      <el-button size="large" @click="resetSettings">重置</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { Setting, Monitor, InfoFilled, FolderOpened } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { useConfigStore } from '@/stores/config'
import { open } from '@tauri-apps/plugin-dialog'

const configStore = useConfigStore()

const isTauri = typeof window !== 'undefined' && '__TAURI__' in window

// 使用 configStore 的响应式数据
const config = configStore

onMounted(() => {
  // 加载配置
  configStore.load()
})

const selectDownloadPath = async () => {
  if (isTauri) {
    try {
      const path = await open({ directory: true, multiple: false })
      if (path) {
        configStore.downloadPath = path
      }
    } catch (error) {
      console.error('选择目录失败:', error)
      ElMessage.error('选择目录失败')
    }
  } else {
    ElMessage.info('浏览器环境暂不支持选择目录')
  }
}

const saveSettings = async () => {
  try {
    await configStore.save()
    ElMessage.success('设置已保存')
  } catch (error) {
    console.error('保存设置失败:', error)
    ElMessage.error('保存设置失败')
  }
}

const resetSettings = async () => {
  try {
    configStore.resetToDefaults()
    await configStore.save()
    ElMessage.info('设置已重置')
  } catch (error) {
    console.error('重置设置失败:', error)
    ElMessage.error('重置设置失败')
  }
}
</script>

<style scoped>
.settings {
  padding: 0;
}

.page-title {
  margin-bottom: 20px;
  font-size: 24px;
  font-weight: bold;
  color: #303133;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.setting-card {
  min-height: 350px;
}

.about-info {
  text-align: center;
  padding: 20px;
}

.about-info h3 {
  margin-bottom: 16px;
  color: #303133;
}

.about-info p {
  margin-bottom: 8px;
  color: #606266;
}

.actions {
  margin-top: 20px;
  text-align: center;
}
</style>
