import { createRouter, createWebHistory, createWebHashHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

// Tauri 通过自定义协议（tauri://）加载页面，没有真正的 web 服务器，
// HTML5 history 模式会导致路由无法解析、页面空白，因此使用 hash 模式。
const isTauri = typeof window !== 'undefined' && '__TAURI__' in window
const history = isTauri ? createWebHashHistory() : createWebHistory()

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
        meta: { requiresAdmin: true },
      },
      {
        path: 'software',
        name: 'AdminSoftware',
        component: () => import('@/views/admin/AdminSoftware.vue'),
        meta: { requiresAdmin: true },
      },
      {
        path: 'categories',
        name: 'AdminCategories',
        component: () => import('@/views/admin/AdminCategories.vue'),
        meta: { requiresAdmin: true },
      },
      {
        path: 'blacklists',
        name: 'AdminBlacklists',
        component: () => import('@/views/admin/AdminBlacklists.vue'),
        meta: { requiresAdmin: true },
      },
      {
        path: 'blacklist-scan',
        name: 'AdminBlacklistScan',
        component: () => import('@/views/admin/AdminBlacklistScan.vue'),
        meta: { requiresAdmin: true },
      },
      {
        path: 'reports',
        name: 'AdminReports',
        component: () => import('@/views/admin/AdminReports.vue'),
        meta: { requiresAdmin: true },
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
  history,
  routes,
})

// 路由守卫
// apphub-client/src/router/index.ts
router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore()

  // ✅ 修复：使用 await 确保用户信息加载完成后再进行权限判断
  if (authStore.token && !authStore.userInfo && !authStore.loading) {
    authStore.loading = true
    try {
      await authStore.fetchUserInfo()
    } catch (error) {
      // 加载失败时清理状态并跳转登录
      authStore.token = null
      authStore.refreshToken = null
      localStorage.removeItem('token')
      localStorage.removeItem('refreshToken')
      if (to.path !== '/login') {
        return next('/login')
      }
    } finally {
      authStore.loading = false
    }
  }

  // 此时 userInfo 已加载完毕，isAdmin 判断是准确的
  if (to.meta.public) {
    next()
  } else if (!authStore.isLoggedIn) {
    next('/login')
  } else if (to.meta.requiresAdmin && !authStore.isAdmin) {
    next('/') // 或者 next('/login')，取决于你的业务逻辑
  } else if (authStore.isAdmin && to.path === '/') {
    next('/admin')
  } else {
    next()
  }
})

// router.beforeEach((to, from, next) => {
//   const authStore = useAuthStore()

//   // 如果有 token 但用户信息未加载，先放行页面渲染，后台加载用户信息
//   if (authStore.token && !authStore.userInfo && !authStore.loading) {
//     authStore.loading = true
//     // 后台加载用户信息，不阻塞路由
//     authStore.fetchUserInfo().finally(() => {
//       authStore.loading = false
//       // 如果加载失败且不在登录页，跳转到登录页
//       if (!authStore.userInfo && to.path !== '/login') {
//         authStore.token = null
//         authStore.refreshToken = null
//         localStorage.removeItem('token')
//         localStorage.removeItem('refreshToken')
//         next('/login')
//       }
//     })
//   }


//   if (to.meta.public) {
//     // 公开页面（如登录页），已登录则跳转首页
//     if (authStore.isLoggedIn) {
//       next(authStore.isAdmin ? '/admin' : '/')
//     } else {
//       next()
//     }
//   } else if (to.meta.requiresAuth && !authStore.isLoggedIn) {
//     // 需要认证但未登录
//     next('/login')
//   } else if (to.meta.requiresAdmin && !authStore.isAdmin) {
//     // 需要管理员权限但不是管理员
//     next('/')
//   } else if (authStore.isAdmin && to.path.startsWith('/') && !to.path.startsWith('/admin')) {
//     // 管理员访问普通用户页面，重定向到管理后台
//     next('/admin')
//   } else {
//     next()
//   }
// })

export default router
