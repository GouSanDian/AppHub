<template>
  <div class="download">
    <h2 class="page-title">下载管理</h2>

    <el-card>
      <template #header>
        <div class="card-header">
          <span>下载任务</span>
          <el-button type="primary" :icon="Plus">新建下载</el-button>
        </div>
      </template>

      <el-empty description="暂无下载任务" />

      <!-- 下载列表 -->
      <el-table v-if="false" :data="[]" style="width: 100%">
        <el-table-column prop="name" label="文件名" />
        <el-table-column prop="size" label="大小" width="120" />
        <el-table-column prop="progress" label="进度" width="200">
          <template #default="{ row }">
            <el-progress :percentage="row.progress" />
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)">
              {{ getStatusText(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="150">
          <template #default="{ row }">
            <el-button-group>
              <el-button size="small" :icon="VideoPlay" v-if="row.status === 'paused'" />
              <el-button size="small" :icon="VideoPause" v-if="row.status === 'downloading'" />
              <el-button size="small" :icon="Delete" type="danger" />
            </el-button-group>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { Plus, VideoPlay, VideoPause, Delete } from '@element-plus/icons-vue'

const getStatusType = (status: string) => {
  const types: Record<string, string> = {
    pending: 'info',
    downloading: 'primary',
    paused: 'warning',
    completed: 'success',
    failed: 'danger',
  }
  return types[status] || 'info'
}

const getStatusText = (status: string) => {
  const texts: Record<string, string> = {
    pending: '等待中',
    downloading: '下载中',
    paused: '已暂停',
    completed: '已完成',
    failed: '失败',
  }
  return texts[status] || status
}
</script>

<style scoped>
.download {
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
  justify-content: space-between;
  align-items: center;
}
</style>
