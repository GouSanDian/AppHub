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
              />
            </div>
          </template>

          <el-row :gutter="20">
            <el-col :span="12">
              <h3>下载统计</h3>
              <el-table :data="downloadStats" style="width: 100%">
                <el-table-column prop="software_name" label="软件名称" />
                <el-table-column prop="download_count" label="下载次数" />
                <el-table-column prop="date" label="日期" />
              </el-table>
            </el-col>
            <el-col :span="12">
              <h3>用户活跃度</h3>
              <el-table :data="userStats" style="width: 100%">
                <el-table-column prop="username" label="用户名" />
                <el-table-column prop="login_count" label="登录次数" />
                <el-table-column prop="last_login" label="最后登录" />
              </el-table>
            </el-col>
          </el-row>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const dateRange = ref<[Date, Date]>()

const downloadStats = ref([
  { software_name: '软件A', download_count: 120, date: '2024-01-15' },
  { software_name: '软件B', download_count: 85, date: '2024-01-15' },
])

const userStats = ref([
  { username: 'admin', login_count: 25, last_login: '2024-01-15 10:30' },
  { username: 'user1', login_count: 12, last_login: '2024-01-14 16:20' },
])
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
</style>
