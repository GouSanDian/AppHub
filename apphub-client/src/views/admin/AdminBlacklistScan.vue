<template>
  <div class="admin-blacklist-scan">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>黑名单扫描记录</span>
        </div>
      </template>

      <!-- 搜索区域 -->
      <div class="filter-bar">
        <el-input
          v-model="searchKeyword"
          placeholder="搜索用户名/进程名..."
          clearable
          prefix-icon="Search"
          style="width: 280px"
          @input="debouncedSearch"
          @clear="handleSearch"
        />
        <el-button @click="handleReset">重置</el-button>
      </div>

      <el-table :data="scanRecords" v-loading="loading" style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="username" label="用户名" width="150" />
        <el-table-column prop="process_name" label="黑名单进程" />
        <el-table-column prop="risk_level" label="风险等级" width="100">
          <template #default="{ row }">
            <el-tag :type="riskLevelTag(row.risk_level)">{{ riskLevelText(row.risk_level) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="scan_time" label="扫描时间" width="180">
          <template #default="{ row }">
            {{ formatTime(row.scan_time) }}
          </template>
        </el-table-column>
        <el-table-column prop="client_id" label="客户端ID" width="200" show-overflow-tooltip />
      </el-table>

      <!-- 分页 -->
      <div class="pagination-bar">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[20, 50, 100]"
          :total="total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSearch"
          @current-change="handleSearch"
        />
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import request from '@/utils/request'

interface ScanRecord {
  id: number
  client_id: string
  user_id: number
  username: string
  process_name: string
  risk_level: number
  scan_time: string
  created_at: string
}

const scanRecords = ref<ScanRecord[]>([])
const loading = ref(false)

// 搜索状态
const searchKeyword = ref('')
const currentPage = ref(1)
const pageSize = ref(20)
const total = ref(0)

// debounce timer
let debounceTimer: ReturnType<typeof setTimeout> | null = null

const debouncedSearch = () => {
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    handleSearch()
  }, 300)
}

onMounted(() => {
  loadScanRecords()
})

const loadScanRecords = async () => {
  loading.value = true
  try {
    const params: Record<string, any> = {
      page: currentPage.value,
      page_size: pageSize.value,
    }
    if (searchKeyword.value.trim()) {
      params.keyword = searchKeyword.value.trim()
    }
    const response = await request.get<any, any>('/reports/scan-records', { params })
    scanRecords.value = response.data?.list || []
    total.value = response.data?.total || 0
  } catch (error) {
    ElMessage.error('加载扫描记录失败')
  } finally {
    loading.value = false
  }
}

const handleSearch = () => {
  currentPage.value = 1
  loadScanRecords()
}

const handleReset = () => {
  searchKeyword.value = ''
  currentPage.value = 1
  loadScanRecords()
}

const riskLevelText = (level: number) => {
  const map: Record<number, string> = { 1: '低', 2: '中', 3: '高' }
  return map[level] || '未知'
}

const riskLevelTag = (level: number) => {
  const map: Record<number, string> = { 1: 'info', 2: 'warning', 3: 'danger' }
  return map[level] || 'info'
}

const formatTime = (timeStr: string) => {
  try {
    const date = new Date(timeStr)
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    })
  } catch {
    return timeStr
  }
}
</script>

<style scoped>
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.filter-bar {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
  align-items: center;
  flex-wrap: wrap;
}
.pagination-bar {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}
</style>
