<template>
  <div class="admin-dashboard">
    <el-row :gutter="20">
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon" style="background-color: #409EFF;">
              <el-icon :size="32"><User /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.userCount }}</div>
              <div class="stat-label">用户总数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon" style="background-color: #67C23A;">
              <el-icon :size="32"><Files /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.softwareCount }}</div>
              <div class="stat-label">软件总数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon" style="background-color: #E6A23C;">
              <el-icon :size="32"><Download /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.downloadCount }}</div>
              <div class="stat-label">下载次数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon" style="background-color: #F56C6C;">
              <el-icon :size="32"><Warning /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.blacklistCount }}</div>
              <div class="stat-label">黑名单数量</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" style="margin-top: 20px;">
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>最近登录</span>
            </div>
          </template>
          <el-table :data="recentLogins" style="width: 100%">
            <el-table-column prop="username" label="用户名" />
            <el-table-column prop="login_time" label="登录时间" />
          </el-table>
        </el-card>
      </el-col>
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>系统信息</span>
            </div>
          </template>
          <el-descriptions :column="1" border>
            <el-descriptions-item label="系统版本">v1.0.0</el-descriptions-item>
            <el-descriptions-item label="后端服务">
              <el-tag type="success">运行中</el-tag>
            </el-descriptions-item>
            <el-descriptions-item label="数据库">
              <el-tag type="success">已连接</el-tag>
            </el-descriptions-item>
          </el-descriptions>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { User, Files, Download, Warning } from '@element-plus/icons-vue'
import request from '@/utils/request'

const stats = ref({
  userCount: 0,
  softwareCount: 0,
  downloadCount: 0,
  blacklistCount: 0,
})

const recentLogins = ref<Array<{ username: string; login_time: string }>>([])

onMounted(async () => {
  await loadStats()
})

const loadStats = async () => {
  try {
    // 调用统计接口
    const res = await request.get<any, any>('/reports/statistics')
    const data = res.data || {}

    stats.value.userCount = data.userCount || 0
    stats.value.softwareCount = data.softwareCount || 0
    stats.value.downloadCount = data.downloadCount || 0
    stats.value.blacklistCount = data.blacklistCount || 0

    // 最近登录
    recentLogins.value = data.recentLogins || []
  } catch (error) {
    console.error('加载统计数据失败:', error)
  }
}
</script>

<style scoped>
.admin-dashboard {
  padding: 0;
}

.stat-card {
  margin-bottom: 0;
}

.stat-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  width: 64px;
  height: 64px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
}

.stat-info {
  flex: 1;
}

.stat-value {
  font-size: 28px;
  font-weight: bold;
  color: #303133;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 14px;
  color: #909399;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
