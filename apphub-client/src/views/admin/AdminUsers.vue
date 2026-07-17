<template>
  <div class="admin-users">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>用户管理</span>
          <el-button type="primary" @click="handleAdd" :disabled="!canManage">
            <el-icon><Plus /></el-icon>
            新增用户
          </el-button>
        </div>
      </template>

      <!-- 搜索栏 -->
      <div class="search-bar">
        <el-input
          v-model="searchQuery"
          placeholder="搜索用户名、昵称或邮箱"
          clearable
          style="width: 300px"
          @clear="handleSearch"
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
        <el-select v-model="roleFilter" placeholder="角色筛选" clearable style="width: 150px" @change="handleSearch">
          <el-option label="全部" value="" />
          <el-option label="超级管理员" value="super_admin" />
          <el-option label="管理员" value="admin" />
          <el-option label="普通用户" value="user" />
        </el-select>
        <el-select v-model="statusFilter" placeholder="状态筛选" clearable style="width: 150px" @change="handleSearch">
          <el-option label="全部" value="" />
          <el-option label="启用" :value="1" />
          <el-option label="禁用" :value="0" />
        </el-select>
        <el-button type="primary" @click="handleSearch">
          <el-icon><Search /></el-icon>
          搜索
        </el-button>
        <el-button @click="handleReset">
          <el-icon><Refresh /></el-icon>
          重置
        </el-button>
      </div>

      <!-- 用户表格 -->
      <el-table :data="filteredUsers" v-loading="loading" style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="username" label="用户名" min-width="120" />
        <el-table-column prop="nickname" label="昵称" min-width="120" />
        <el-table-column prop="email" label="邮箱" min-width="180" />
        <el-table-column prop="role" label="角色" width="120">
          <template #default="{ row }">
            <el-tag :type="getRoleTagType(row.role)">
              {{ getRoleLabel(row.role) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="row.status === 1 ? 'success' : 'warning'">
              {{ row.status === 1 ? '启用' : '禁用' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="created_at" label="创建时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="280" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="handleEdit(row)" :disabled="!canManage">
              编辑
            </el-button>
            <el-button
              size="small"
              type="warning"
              @click="handleResetPassword(row)"
              :disabled="!canManage"
            >
              重置密码
            </el-button>
            <el-button
              size="small"
              :type="row.status === 1 ? 'danger' : 'success'"
              @click="handleToggleStatus(row)"
              :disabled="!canManage || row.role === 'super_admin'"
            >
              {{ row.status === 1 ? '禁用' : '启用' }}
            </el-button>
            <el-button
              size="small"
              type="danger"
              @click="handleDelete(row)"
              :disabled="!isSuperAdmin || row.role === 'super_admin'"
            >
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination-wrapper">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handlePageChange"
        />
      </div>
    </el-card>

    <!-- 新增/编辑用户对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogType === 'add' ? '新增用户' : '编辑用户'"
      width="500px"
    >
      <el-form
        ref="formRef"
        :model="formData"
        :rules="formRules"
        label-width="100px"
      >
        <el-form-item label="用户名" prop="username">
          <el-input
            v-model="formData.username"
            placeholder="请输入用户名"
            :disabled="dialogType === 'edit'"
          />
        </el-form-item>
        <el-form-item label="密码" prop="password" v-if="dialogType === 'add'">
          <el-input
            v-model="formData.password"
            type="password"
            placeholder="请输入密码"
            show-password
          />
        </el-form-item>
        <el-form-item label="昵称" prop="nickname">
          <el-input v-model="formData.nickname" placeholder="请输入昵称" />
        </el-form-item>
        <el-form-item label="邮箱" prop="email">
          <el-input v-model="formData.email" placeholder="请输入邮箱" />
        </el-form-item>
        <el-form-item label="角色" prop="role_id">
          <el-select v-model="formData.role_id" placeholder="请选择角色" style="width: 100%">
            <el-option label="超级管理员" :value="1" :disabled="!isSuperAdmin" />
            <el-option label="管理员" :value="2" />
            <el-option label="普通用户" :value="3" />
          </el-select>
        </el-form-item>
        <el-form-item label="状态" prop="status" v-if="dialogType === 'edit'">
          <el-radio-group v-model="formData.status">
            <el-radio :label="1">启用</el-radio>
            <el-radio :label="0">禁用</el-radio>
          </el-radio-group>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSubmit" :loading="submitLoading">
          确定
        </el-button>
      </template>
    </el-dialog>

    <!-- 重置密码对话框 -->
    <el-dialog
      v-model="resetPasswordVisible"
      title="重置密码"
      width="500px"
    >
      <el-form
        ref="resetFormRef"
        :model="resetFormData"
        :rules="resetFormRules"
        label-width="100px"
      >
        <el-form-item label="用户名">
          <el-input :value="currentUser?.username" disabled />
        </el-form-item>
        <el-form-item label="新密码" prop="password">
          <el-input
            v-model="resetFormData.password"
            type="password"
            placeholder="请输入新密码"
            show-password
          />
        </el-form-item>
        <el-form-item label="确认密码" prop="confirmPassword">
          <el-input
            v-model="resetFormData.confirmPassword"
            type="password"
            placeholder="请再次输入密码"
            show-password
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="resetPasswordVisible = false">取消</el-button>
        <el-button type="primary" @click="handleResetPasswordSubmit" :loading="resetLoading">
          确定
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus'
import { Plus, Search, Refresh } from '@element-plus/icons-vue'
import request from '@/utils/request'
import { useAuthStore } from '@/stores/auth'

interface User {
  id: number
  username: string
  nickname: string
  email: string
  role: string
  role_id: number
  status: number
  created_at: string
}

const authStore = useAuthStore()

// 权限控制
const canManage = computed(() => authStore.isAdmin)
const isSuperAdmin = computed(() => authStore.isSuperAdmin)

// 列表数据
const users = ref<User[]>([])
const loading = ref(false)
const currentPage = ref(1)
const pageSize = ref(10)
const total = ref(0)

// 搜索和筛选
const searchQuery = ref('')
const roleFilter = ref('')
const statusFilter = ref('')

// 对话框
const dialogVisible = ref(false)
const dialogType = ref<'add' | 'edit'>('add')
const formRef = ref<FormInstance>()
const submitLoading = ref(false)
const currentUser = ref<User | null>(null)

const formData = ref({
  username: '',
  password: '',
  nickname: '',
  email: '',
  role_id: 3,
  status: 1,
})

const formRules: FormRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '长度在 3 到 20 个字符', trigger: 'blur' },
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码长度不能少于 6 个字符', trigger: 'blur' },
  ],
  email: [
    { type: 'email', message: '请输入正确的邮箱地址', trigger: 'blur' },
  ],
  role_id: [
    { required: true, message: '请选择角色', trigger: 'change' },
  ],
}

// 重置密码
const resetPasswordVisible = ref(false)
const resetFormRef = ref<FormInstance>()
const resetLoading = ref(false)
const resetFormData = ref({
  password: '',
  confirmPassword: '',
})

const resetFormRules: FormRules = {
  password: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, message: '密码长度不能少于 6 个字符', trigger: 'blur' },
  ],
  confirmPassword: [
    { required: true, message: '请再次输入密码', trigger: 'blur' },
    {
      validator: (rule, value, callback) => {
        if (value !== resetFormData.value.password) {
          callback(new Error('两次输入的密码不一致'))
        } else {
          callback()
        }
      },
      trigger: 'blur',
    },
  ],
}

// 计算属性：过滤后的用户列表
const filteredUsers = computed(() => {
  let result = users.value

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(
      user =>
        user.username.toLowerCase().includes(query) ||
        user.nickname?.toLowerCase().includes(query) ||
        user.email?.toLowerCase().includes(query)
    )
  }

  // 角色过滤
  if (roleFilter.value) {
    result = result.filter(user => user.role === roleFilter.value)
  }

  // 状态过滤
  if (statusFilter.value !== '') {
    result = result.filter(user => user.status === statusFilter.value)
  }

  return result
})

onMounted(() => {
  loadUsers()
})

// 加载用户列表
const loadUsers = async () => {
  loading.value = true
  try {
    const response = await request.get<any, any>('/users', {
      params: {
        page: currentPage.value,
        page_size: pageSize.value,
      },
    })
    const data = response.data
    users.value = data.list || []
    total.value = data.total || 0
  } catch (error) {
    ElMessage.error('加载用户列表失败')
  } finally {
    loading.value = false
  }
}

// 搜索
const handleSearch = () => {
  currentPage.value = 1
  loadUsers()
}

// 重置搜索
const handleReset = () => {
  searchQuery.value = ''
  roleFilter.value = ''
  statusFilter.value = ''
  currentPage.value = 1
  loadUsers()
}

// 分页
const handleSizeChange = () => {
  currentPage.value = 1
  loadUsers()
}

const handlePageChange = () => {
  loadUsers()
}

// 新增用户
const handleAdd = () => {
  if (!canManage.value) {
    ElMessage.warning('您没有权限执行此操作')
    return
  }
  dialogType.value = 'add'
  formData.value = {
    username: '',
    password: '',
    nickname: '',
    email: '',
    role_id: 3,
    status: 1,
  }
  dialogVisible.value = true
  formRef.value?.clearValidate()
}

// 编辑用户
const handleEdit = (row: User) => {
  if (!canManage.value) {
    ElMessage.warning('您没有权限执行此操作')
    return
  }
  dialogType.value = 'edit'
  currentUser.value = row
  formData.value = {
    username: row.username,
    password: '',
    nickname: row.nickname,
    email: row.email,
    role_id: row.role_id,
    status: row.status,
  }
  dialogVisible.value = true
  formRef.value?.clearValidate()
}

// 提交表单
const handleSubmit = async () => {
  if (!formRef.value) return

  await formRef.value.validate(async (valid) => {
    if (!valid) return

    submitLoading.value = true
    try {
      if (dialogType.value === 'add') {
        // 新增用户
        await request.post('/users', {
          username: formData.value.username,
          password: formData.value.password,
          nickname: formData.value.nickname || null,
          email: formData.value.email || null,
          role_id: formData.value.role_id,
        })
        ElMessage.success('创建用户成功')
      } else {
        // 编辑用户
        if (!currentUser.value) return
        await request.put(`/users/${currentUser.value.id}`, {
          nickname: formData.value.nickname || null,
          email: formData.value.email || null,
          role_id: formData.value.role_id,
          status: formData.value.status,
        })
        ElMessage.success('更新用户成功')
      }
      dialogVisible.value = false
      await loadUsers()
    } catch (error: any) {
      ElMessage.error(error.message || '操作失败')
    } finally {
      submitLoading.value = false
    }
  })
}

// 重置密码
const handleResetPassword = (row: User) => {
  if (!canManage.value) {
    ElMessage.warning('您没有权限执行此操作')
    return
  }
  currentUser.value = row
  resetFormData.value = {
    password: '',
    confirmPassword: '',
  }
  resetPasswordVisible.value = true
  resetFormRef.value?.clearValidate()
}

// 提交重置密码
const handleResetPasswordSubmit = async () => {
  if (!resetFormRef.value || !currentUser.value) return

  await resetFormRef.value.validate(async (valid) => {
    if (!valid) return

    resetLoading.value = true
    try {
      await request.post(`/users/${currentUser.value.id}/reset-password`, {
        new_password: resetFormData.value.password,
      })
      ElMessage.success('重置密码成功')
      resetPasswordVisible.value = false
    } catch (error: any) {
      ElMessage.error(error.message || '重置密码失败')
    } finally {
      resetLoading.value = false
    }
  })
}

// 切换用户状态
const handleToggleStatus = async (row: User) => {
  if (!canManage.value) {
    ElMessage.warning('您没有权限执行此操作')
    return
  }

  const action = row.status === 1 ? '禁用' : '启用'
  try {
    await ElMessageBox.confirm(
      `确认要${action}用户 ${row.username} 吗？`,
      '提示',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    await request.put(`/users/${row.id}`, {
      status: row.status === 1 ? 0 : 1,
    })
    ElMessage.success(`${action}成功`)
    await loadUsers()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error(`${action}失败`)
    }
  }
}

// 删除用户
const handleDelete = async (row: User) => {
  if (!isSuperAdmin.value) {
    ElMessage.warning('只有超级管理员才能删除用户')
    return
  }

  if (row.role === 'super_admin') {
    ElMessage.warning('不能删除超级管理员')
    return
  }

  try {
    await ElMessageBox.confirm(
      `确认要删除用户 ${row.username} 吗？此操作不可恢复！`,
      '警告',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    await request.delete(`/users/${row.id}`)
    ElMessage.success('删除成功')
    await loadUsers()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

// 辅助函数
const getRoleTagType = (role: string): '' | 'success' | 'warning' | 'danger' | 'info' => {
  switch (role) {
    case 'super_admin':
      return 'danger'
    case 'admin':
      return 'warning'
    case 'user':
      return 'info'
    default:
      return 'info'
  }
}

const getRoleLabel = (role: string): string => {
  switch (role) {
    case 'super_admin':
      return '超级管理员'
    case 'admin':
      return '管理员'
    case 'user':
      return '普通用户'
    default:
      return role
  }
}

const formatDate = (dateStr: string): string => {
  if (!dateStr) return ''
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  })
}
</script>

<style scoped>
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.search-bar {
  display: flex;
  gap: 12px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

.pagination-wrapper {
  margin-top: 20px;
  display: flex;
  justify-content: flex-end;
}
</style>
