<template>
  <div class="dashboard">
    <h2 class="page-title">首页</h2>

    <el-row :gutter="20" class="content-row">
      <el-col :xs="24" :lg="12">
        <el-card class="content-card">
          <template #header>
            <div class="card-header">
              <span>最近更新</span>
              <el-button link type="primary" @click="goSoftwareCenter">查看更多</el-button>
            </div>
          </template>
          <div v-loading="loading">
            <div v-if="recentList.length === 0 && !loading" class="empty-wrap">
              <el-empty description="暂无数据" />
            </div>
            <ul v-else class="recent-list">
              <li
                v-for="item in recentList"
                :key="item.id"
                class="recent-item"
              >
                <div class="recent-icon">
                  <el-icon :size="24"><Box /></el-icon>
                </div>
                <div class="recent-info">
                  <div class="recent-name">{{ item.name }}</div>
                  <div class="recent-meta">
                    <span>v{{ item.version }}</span>
                    <span v-if="item.category_name" class="recent-category">· {{ item.category_name }}</span>
                  </div>
                </div>
                <div class="recent-time">{{ formatTime(item.updated_at || item.created_at) }}</div>
              </li>
            </ul>
          </div>
        </el-card>
      </el-col>

      <el-col :xs="24" :lg="12">
        <el-card class="content-card">
          <template #header>
            <div class="card-header">
              <span>安全状态</span>
              <el-tag type="success">正常</el-tag>
            </div>
          </template>
          <div class="security-status">
            <el-result
              icon="success"
              title="系统安全"
              sub-title="未发现黑名单进程"
            />
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Box } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import request from '@/utils/request'

interface RecentSoftware {
  id: number
  name: string
  version: string
  category_name?: string
  updated_at?: string
  created_at?: string
}

const router = useRouter()
const recentList = ref<RecentSoftware[]>([])
const loading = ref(false)

const goSoftwareCenter = () => {
  router.push('/software')
}

const loadRecentSoftware = async () => {
  loading.value = true
  try {
    const response = await request.get<any, any>('/softwares', {
      params: { limit: 5, order: 'recent' },
    })
    const list: RecentSoftware[] = response.data?.list || []
    recentList.value = list.slice(0, 5)
  } catch (error) {
    ElMessage.error('加载最近更新失败')
  } finally {
    loading.value = false
  }
}

const formatTime = (time?: string) => {
  if (!time) return ''
  const date = new Date(time)
  if (Number.isNaN(date.getTime())) return ''
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMin = Math.floor(diffMs / 60000)
  const diffHour = Math.floor(diffMs / 3600000)
  const diffDay = Math.floor(diffMs / 86400000)
  if (diffMin < 1) return '刚刚'
  if (diffMin < 60) return `${diffMin} 分钟前`
  if (diffHour < 24) return `${diffHour} 小时前`
  if (diffDay < 30) return `${diffDay} 天前`
  const y = date.getFullYear()
  const m = String(date.getMonth() + 1).padStart(2, '0')
  const d = String(date.getDate()).padStart(2, '0')
  return `${y}-${m}-${d}`
}

onMounted(() => {
  loadRecentSoftware()
})
</script>

<style scoped>
.dashboard {
  padding: 0;
}

.page-title {
  margin-bottom: 20px;
  font-size: 24px;
  font-weight: bold;
  color: #303133;
}

.content-row {
  margin-top: 0;
}

.content-card {
  min-height: 300px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.empty-wrap {
  padding: 40px 0;
}

.security-status {
  padding: 40px 0;
}

.recent-list {
  list-style: none;
  margin: 0;
  padding: 0;
}

.recent-item {
  display: flex;
  align-items: center;
  padding: 12px 4px;
  border-bottom: 1px solid #f0f2f5;
  transition: background-color 0.2s;
}

.recent-item:last-child {
  border-bottom: none;
}

.recent-item:hover {
  background-color: #f5f7fa;
}

.recent-icon {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  background-color: #ecf5ff;
  color: #409eff;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  margin-right: 12px;
}

.recent-info {
  flex: 1;
  min-width: 0;
}

.recent-name {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.recent-meta {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
}

.recent-category {
  margin-left: 4px;
}

.recent-time {
  font-size: 12px;
  color: #909399;
  flex-shrink: 0;
  margin-left: 12px;
}
</style>
