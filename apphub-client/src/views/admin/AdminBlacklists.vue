<template>
  <div class="admin-blacklists">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>黑名单管理</span>
          <el-button type="primary" @click="handleAdd">新增黑名单</el-button>
        </div>
      </template>

      <!-- 搜索过滤区域 -->
      <div class="filter-bar">
        <el-input
          v-model="searchKeyword"
          placeholder="搜索进程名/描述..."
          clearable
          prefix-icon="Search"
          style="width: 280px"
          @input="debouncedSearch"
          @clear="handleSearch"
        />
        <el-select
          v-model="filterRiskLevel"
          placeholder="风险等级"
          clearable
          style="width: 140px"
          @change="handleSearch"
        >
          <el-option label="低 (1)" :value="1" />
          <el-option label="中 (2)" :value="2" />
          <el-option label="高 (3)" :value="3" />
        </el-select>
        <el-select
          v-model="filterStatus"
          placeholder="状态"
          clearable
          style="width: 120px"
          @change="handleSearch"
        >
          <el-option label="启用" :value="1" />
          <el-option label="禁用" :value="0" />
        </el-select>
        <el-button @click="handleReset">重置</el-button>
      </div>

      <!-- 新增/编辑对话框 -->
      <el-dialog
        v-model="dialogVisible"
        :title="isEdit ? '编辑黑名单' : '新增黑名单'"
        width="500px"
      >
        <el-form :model="form" label-width="100px">
          <el-form-item label="进程名" required>
            <el-input
              v-model="form.process_name"
              placeholder="请输入进程名，如 game.exe"
              :disabled="isEdit"
            />
          </el-form-item>
          <el-form-item label="描述">
            <el-input
              v-model="form.description"
              type="textarea"
              :rows="3"
              placeholder="请输入描述"
            />
          </el-form-item>
          <el-form-item label="风险等级">
            <el-select v-model="form.risk_level" style="width: 100%">
              <el-option label="低 (1)" :value="1" />
              <el-option label="中 (2)" :value="2" />
              <el-option label="高 (3)" :value="3" />
            </el-select>
          </el-form-item>
        </el-form>
        <template #footer>
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSubmit">确定</el-button>
        </template>
      </el-dialog>

      <el-table :data="filteredBlacklists" v-loading="loading" style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="process_name" label="进程名" />
        <el-table-column prop="description" label="描述" />
        <el-table-column prop="risk_level" label="风险等级" width="100">
          <template #default="{ row }">
            <el-tag :type="riskLevelTag(row.risk_level)">{{ riskLevelText(row.risk_level) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="80">
          <template #default="{ row }">
            <el-tag :type="row.status === 1 ? 'success' : 'info'">{{ row.status === 1 ? '启用' : '禁用' }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="created_at" label="添加时间" width="180" />
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="handleEdit(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination-bar">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50]"
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
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import request from '@/utils/request'

interface Blacklist {
  id: number
  process_name: string
  description: string
  risk_level: number
  status: number
  created_at: string
}

const blacklists = ref<Blacklist[]>([])
const loading = ref(false)

// 搜索过滤状态
const searchKeyword = ref('')
const filterRiskLevel = ref<number | undefined>(undefined)
const filterStatus = ref<number | undefined>(undefined)
const currentPage = ref(1)
const pageSize = ref(10)
const total = ref(0)

// 对话框状态
const dialogVisible = ref(false)
const isEdit = ref(false)
const form = ref<{
  id?: number
  process_name: string
  description: string
  risk_level: number
}>({
  process_name: '',
  description: '',
  risk_level: 1,
})

// debounce timer
let debounceTimer: ReturnType<typeof setTimeout> | null = null

const debouncedSearch = () => {
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    handleSearch()
  }, 300)
}

onMounted(() => {
  loadBlacklists()
})

const loadBlacklists = async () => {
  loading.value = true
  try {
    const params: Record<string, any> = {
      page: currentPage.value,
      page_size: pageSize.value,
    }
    if (searchKeyword.value.trim()) {
      params.keyword = searchKeyword.value.trim()
    }
    if (filterRiskLevel.value !== undefined) {
      params.risk_level = filterRiskLevel.value
    }
    if (filterStatus.value !== undefined) {
      params.status = filterStatus.value
    }
    const response = await request.get<any, any>('/blacklists', { params })
    blacklists.value = response.data?.list || []
    total.value = response.data?.total || 0
  } catch (error) {
    ElMessage.error('加载黑名单列表失败')
  } finally {
    loading.value = false
  }
}

// 前端过滤（当后端尚未实现搜索时作为兜底）
const filteredBlacklists = computed(() => {
  // 如果后端返回了 total > 0 或 list 长度等于 total，说明后端已实现过滤，直接用
  if (total.value > 0 && blacklists.value.length <= pageSize.value) {
    return blacklists.value
  }
  // 否则前端兜底过滤
  let result = blacklists.value
  const kw = searchKeyword.value.trim().toLowerCase()
  if (kw) {
    result = result.filter(
      (item) =>
        item.process_name.toLowerCase().includes(kw) ||
        (item.description && item.description.toLowerCase().includes(kw))
    )
  }
  if (filterRiskLevel.value !== undefined) {
    result = result.filter((item) => item.risk_level === filterRiskLevel.value)
  }
  if (filterStatus.value !== undefined) {
    result = result.filter((item) => item.status === filterStatus.value)
  }
  total.value = result.length
  // 前端分页
  const start = (currentPage.value - 1) * pageSize.value
  return result.slice(start, start + pageSize.value)
})

const handleSearch = () => {
  currentPage.value = 1
  loadBlacklists()
}

const handleReset = () => {
  searchKeyword.value = ''
  filterRiskLevel.value = undefined
  filterStatus.value = undefined
  currentPage.value = 1
  loadBlacklists()
}

const riskLevelText = (level: number) => {
  const map: Record<number, string> = { 1: '低', 2: '中', 3: '高' }
  return map[level] || '未知'
}

const riskLevelTag = (level: number) => {
  const map: Record<number, string> = { 1: 'info', 2: 'warning', 3: 'danger' }
  return map[level] || 'info'
}

const handleAdd = () => {
  dialogVisible.value = true
  isEdit.value = false
  form.value = {
    process_name: '',
    description: '',
    risk_level: 1,
  }
}

const handleEdit = (row: Blacklist) => {
  dialogVisible.value = true
  isEdit.value = true
  form.value = {
    id: row.id,
    process_name: row.process_name,
    description: row.description || '',
    risk_level: row.risk_level,
  }
}

const handleSubmit = async () => {
  if (!form.value.process_name.trim()) {
    ElMessage.warning('请输入进程名')
    return
  }

  try {
    if (isEdit.value && form.value.id) {
      await request.put(`/blacklists/${form.value.id}`, {
        description: form.value.description,
        risk_level: form.value.risk_level,
      })
      ElMessage.success('更新成功')
    } else {
      await request.post('/blacklists', {
        process_name: form.value.process_name,
        description: form.value.description,
        risk_level: form.value.risk_level,
      })
      ElMessage.success('创建成功')
    }
    dialogVisible.value = false
    await loadBlacklists()
  } catch (error: any) {
    if (error.response?.status === 409) {
      ElMessage.error(error.response?.data?.message || '该进程已在黑名单中')
    } else {
      ElMessage.error(isEdit.value ? '更新失败' : '创建失败')
    }
  }
}

const handleDelete = async (row: Blacklist) => {
  try {
    await ElMessageBox.confirm(`确认要删除黑名单 ${row.process_name} 吗？`, '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    })
    await request.delete(`/blacklists/${row.id}`)
    ElMessage.success('删除成功')
    await loadBlacklists()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
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
