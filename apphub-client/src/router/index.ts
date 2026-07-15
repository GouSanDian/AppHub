import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/LoginView.vue'),
    meta: { public: true },
  },
  {
    path: '/',
    name: 'Home',
    component: () => import('@/views/HomeView.vue'),
    meta: { requiresAuth: true },
    children: [
      {
        path: '',
        name: 'Dashboard',
        component: () => import('@/views/DashboardView.vue'),
      },
      {
        path: 'software',
        name: 'Software',
        component: () => import('@/views/SoftwareView.vue'),
      },
      {
        path: 'downloads',
        name: 'Downloads',
        component: () => import('@/views/DownloadView.vue'),
      },
      {
        path: 'settings',
        name: 'Settings',
        component: () => import('@/views/SettingsView.vue'),
      },
    ],
  },
  {
    path: '/admin',
    name: 'Admin',
    component: () => import('@/views/admin/AdminLayout.vue'),
    meta: { requiresAuth: true, requiresAdmin: true },
    children: [
      {
        path: '',
        name: 'AdminDashboard',
        component: () => import('@/views/admin/AdminDashboard.vue'),
      },
      {
        path: 'users',
        name: 'AdminUsers',
        component: () => import('@/views/admin/AdminUsers.vue'),
      },
      {
        path: 'software',
        name: 'AdminSoftware',
        component: () => import('@/views/admin/AdminSoftware.vue'),
      },
      {
        path: 'categories',
        name: 'AdminCategories',
        component: () => import('@/views/admin/AdminCategories.vue'),
      },
      {
        path: 'blacklists',
        name: 'AdminBlacklists',
        component: () => import('@/views/admin/AdminBlacklists.vue'),
      },
      {
        path: 'reports',
        name: 'AdminReports',
        component: () => import('@/views/admin/AdminReports.vue'),
      },
    ],
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    component: () => import('@/views/NotFoundView.vue'),
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

// 路由守卫
router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore()

  // 如果 token 存在但用户信息未加载，先加载用户信息
  if (authStore.token && !authStore.userInfo) {
    await authStore.fetchUserInfo()
  }

  if (to.meta.public) {
    // 公开页面（如登录页），已登录则跳转首页
    if (authStore.isLoggedIn) {
      next(authStore.isAdmin ? '/admin' : '/')
    } else {
      next()
    }
  } else if (to.meta.requiresAuth && !authStore.isLoggedIn) {
    // 需要认证但未登录
    next('/login')
  } else if (to.meta.requiresAdmin && !authStore.isAdmin) {
    // 需要管理员权限但不是管理员
    next('/')
  } else {
    next()
  }
})

export default router
