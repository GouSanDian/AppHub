<template>
  <div class="admin-blacklists">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>黑名单管理</span>
          <el-button type="primary" @click="handleAdd">新增黑名单</el-button>
        </div>
      </template>

      <el-table :data="blacklists" v-loading="loading" style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="software_name" label="软件名称" />
        <el-table-column prop="reason" label="原因" />
        <el-table-column prop="created_at" label="添加时间" />
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="handleEdit(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import request from '@/utils/request'

interface Blacklist {
  id: number
  software_name: string
  reason: string
  created_at: string
}

const blacklists = ref<Blacklist[]>([])
const loading = ref(false)

onMounted(() => {
  loadBlacklists()
})

const loadBlacklists = async () => {
  loading.value = true
  try {
    const response = await request.get<any, any>('/blacklists')
    blacklists.value = response.data || []
  } catch (error) {
    ElMessage.error('加载黑名单列表失败')
  } finally {
    loading.value = false
  }
}

const handleAdd = () => {
  ElMessage.info('新增黑名单功能开发中')
}

const handleEdit = (row: Blacklist) => {
  ElMessage.info(`编辑黑名单: ${row.software_name}`)
}

const handleDelete = async (row: Blacklist) => {
  try {
    await ElMessageBox.confirm(`确认要删除黑名单 ${row.software_name} 吗？`, '提示', {
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
</style>
