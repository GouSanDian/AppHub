<template>
  <div class="admin-reports">
    <el-row :gutter="20">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>统计报表</span>
              <el-date-picker
                v-model="dateRange"
                type="daterange"
                range-separator="至"
                start-placeholder="开始日期"
                end-placeholder="结束日期"
                @change="loadStatistics"
              />
            </div>
          </template>

          <!-- 概览统计 -->
          <el-row :gutter="20" style="margin-bottom: 20px;">
            <el-col :span="6">
              <div class="stat-item">
                <div class="stat-value">{{ overview.userCount }}</div>
                <div class="stat-label">用户总数</div>
              </div>
            </el-col>
            <el-col :span="6">
              <div class="stat-item">
                <div class="stat-value">{{ overview.softwareCount }}</div>
                <div class="stat-label">软件总数</div>
              </div>
            </el-col>
            <el-col :span="6">
              <div class="stat-item">
                <div class="stat-value">{{ overview.downloadCount }}</div>
                <div class="stat-label">总下载次数</div>
              </div>
            </el-col>
            <el-col :span="6">
              <div class="stat-item">
                <div class="stat-value">{{ overview.blacklistCount }}</div>
                <div class="stat-label">黑名单数量</div>
              </div>
            </el-col>
          </el-row>

          <el-row :gutter="20">
            <el-col :span="12">
              <h3>下载统计</h3>
              <el-table :data="downloadStats" v-loading="loading" style="width: 100%">
                <el-table-column prop="name" label="软件名称" />
                <el-table-column prop="download_count" label="下载次数" width="120" />
              </el-table>
            </el-col>
            <el-col :span="12">
              <h3>最近登录</h3>
              <el-table :data="recentLogins" v-loading="loading" style="width: 100%">
                <el-table-column prop="username" label="用户名" />
                <el-table-column prop="login_time" label="登录时间" width="180" />
              </el-table>
            </el-col>
          </el-row>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import request from '@/utils/request'

const dateRange = ref<[Date, Date]>()
const loading = ref(false)

const overview = ref({
  userCount: 0,
  softwareCount: 0,
  downloadCount: 0,
  blacklistCount: 0,
})

const downloadStats = ref<Array<{ name: string; download_count: number }>>([])
const recentLogins = ref<Array<{ username: string; login_time: string }>>([])

onMounted(() => {
  loadStatistics()
})

const loadStatistics = async () => {
  loading.value = true
  try {
    const res = await request.get<any, any>('/reports/statistics')
    const data = res.data || {}

    overview.value.userCount = data.userCount || 0
    overview.value.softwareCount = data.softwareCount || 0
    overview.value.downloadCount = data.downloadCount || 0
    overview.value.blacklistCount = data.blacklistCount || 0

    recentLogins.value = data.recentLogins || []

    // 获取软件下载统计
    const softwareRes = await request.get<any, any>('/softwares', { params: { page_size: 100 } })
    const softwareList = softwareRes.data?.list || []
    downloadStats.value = softwareList
      .map((s: any) => ({ name: s.name, download_count: s.download_count || 0 }))
      .sort((a: any, b: any) => b.download_count - a.download_count)
      .slice(0, 10)
  } catch (error) {
    console.error('加载统计数据失败:', error)
    ElMessage.error('加载统计数据失败')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

h3 {
  margin-top: 0;
  margin-bottom: 16px;
}

.stat-item {
  text-align: center;
  padding: 20px;
  background-color: #f5f7fa;
  border-radius: 4px;
}

.stat-value {
  font-size: 32px;
  font-weight: bold;
  color: #303133;
  margin-bottom: 8px;
}

.stat-label {
  font-size: 14px;
  color: #909399;
}
</style>
