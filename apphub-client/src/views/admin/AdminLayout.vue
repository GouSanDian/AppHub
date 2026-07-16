<template>
  <el-container class="admin-layout">
    <el-aside width="200px" class="admin-aside">
      <div class="admin-logo">
        <h2>管理员后台</h2>
      </div>
      <el-menu
        :default-active="activeMenu"
        router
        background-color="#304156"
        text-color="#bfcbd9"
        active-text-color="#409EFF"
      >
        <el-menu-item index="/admin">
          <el-icon><HomeFilled /></el-icon>
          <span>控制台</span>
        </el-menu-item>
        <el-menu-item index="/admin/users">
          <el-icon><User /></el-icon>
          <span>用户管理</span>
        </el-menu-item>
        <el-menu-item index="/admin/software">
          <el-icon><Files /></el-icon>
          <span>软件管理</span>
        </el-menu-item>
        <el-menu-item index="/admin/categories">
          <el-icon><Menu /></el-icon>
          <span>分类管理</span>
        </el-menu-item>
        <el-menu-item index="/admin/blacklists">
          <el-icon><Warning /></el-icon>
          <span>黑名单管理</span>
        </el-menu-item>
        <el-menu-item index="/admin/blacklist-scan">
          <el-icon><Search /></el-icon>
          <span>黑名单扫描</span>
        </el-menu-item>
        <el-menu-item index="/admin/reports">
          <el-icon><DataLine /></el-icon>
          <span>统计报表</span>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <el-container>
      <el-header class="admin-header">
        <div class="header-left">
          <el-breadcrumb separator="/">
            <el-breadcrumb-item :to="{ path: '/admin' }">管理员后台</el-breadcrumb-item>
            <el-breadcrumb-item>{{ currentRoute }}</el-breadcrumb-item>
          </el-breadcrumb>
        </div>
        <div class="header-right">
          <el-dropdown @command="handleCommand">
            <span class="user-info">
              <el-avatar :size="32" :src="authStore.userInfo?.avatar">
                {{ authStore.nickname?.charAt(0) || 'A' }}
              </el-avatar>
              <span class="username">{{ authStore.nickname || authStore.username }}</span>
              <el-icon class="el-icon--right"><ArrowDown /></el-icon>
            </span>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="logout">退出登录</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </el-header>

      <el-main class="admin-main">
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  HomeFilled,
  User,
  Files,
  Menu,
  Warning,
  DataLine,
  ArrowDown,
  Search,
} from '@element-plus/icons-vue'
import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()

const activeMenu = computed(() => route.path)
const currentRoute = computed(() => {
  const name = route.name as string
  const nameMap: Record<string, string> = {
    AdminDashboard: '控制台',
    AdminUsers: '用户管理',
    AdminSoftware: '软件管理',
    AdminCategories: '分类管理',
    AdminBlacklists: '黑名单管理',
    AdminReports: '统计报表',
  }
  return nameMap[name] || '控制台'
})

const handleCommand = async (command: string) => {
  if (command === 'logout') {
    try {
      await ElMessageBox.confirm('确认要退出登录吗？', '提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      })
      await authStore.logout()
      ElMessage.success('已退出登录')
      router.push('/login')
    } catch {
      // 用户取消
    }
  }
}
</script>

<style scoped>
.admin-layout {
  height: 100vh;
}

.admin-aside {
  background-color: #304156;
  overflow-y: auto;
}

.admin-logo {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #263445;
  color: #fff;
}

.admin-logo h2 {
  margin: 0;
  font-size: 18px;
}

.admin-header {
  background-color: #fff;
  border-bottom: 1px solid #e6e6e6;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
}

.header-right {
  display: flex;
  align-items: center;
}

.user-info {
  display: flex;
  align-items: center;
  cursor: pointer;
}

.username {
  margin-left: 8px;
  font-size: 14px;
}

.admin-main {
  background-color: #f0f2f5;
  padding: 20px;
  overflow-y: auto;
}
</style>
