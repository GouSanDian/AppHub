<template>
  <div class="login-container">
    <el-card class="login-card">
      <template #header>
        <h2 class="login-title">应用中心</h2>
        <p class="login-subtitle">企业软件管理平台</p>
      </template>

      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-position="top"
        @keyup.enter="handleLogin"
      >
        <el-form-item label="服务器地址" prop="serverUrl">
          <el-input
            v-model="form.serverUrl"
            placeholder="http://localhost:8080/api/v1"
            :prefix-icon="Link"
            size="large"
          />
        </el-form-item>

        <el-form-item label="用户名" prop="username">
          <el-input
            v-model="form.username"
            placeholder="请输入用户名"
            :prefix-icon="User"
            size="large"
            autocapitalize="off"
            autocomplete="off"
            spellcheck="false"
          />
        </el-form-item>

        <el-form-item label="密码" prop="password">
          <el-input
            v-model="form.password"
            type="password"
            placeholder="请输入密码"
            :prefix-icon="Lock"
            size="large"
            show-password
          />
        </el-form-item>

        <el-form-item>
          <el-checkbox v-model="form.remember">记住密码</el-checkbox>
        </el-form-item>

        <el-form-item>
          <el-button
            type="primary"
            size="large"
            :loading="authStore.loading"
            @click="handleLogin"
            class="login-button"
          >
            登录
          </el-button>
        </el-form-item>
      </el-form>

      <el-alert
        v-if="authStore.error"
        :title="authStore.error"
        type="error"
        :closable="false"
        show-icon
        class="login-error"
      />
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { User, Lock, Link } from '@element-plus/icons-vue'
import { useAuthStore } from '@/stores/auth'
import { useConfigStore } from '@/stores/config'
import { setBaseURL } from '@/utils/request'
import type { FormInstance, FormRules } from 'element-plus'

const router = useRouter()
const authStore = useAuthStore()
const configStore = useConfigStore()
const formRef = ref<FormInstance>()
const loading = ref(false)

const form = reactive({
  serverUrl: configStore.serverUrl || 'http://localhost:8080/api/v1',
  username: '',
  password: '',
  remember: false,
})

const rules: FormRules = {
  serverUrl: [{ required: true, message: '请输入服务器地址', trigger: 'blur' }],
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }],
}

onMounted(() => {
  // 从 configStore 回填服务器地址
  form.serverUrl = configStore.serverUrl || 'http://localhost:8080/api/v1'
})

const handleLogin = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (!valid) return
    
    loading.value = true // 开启 loading
    try {
      const success = await authStore.login(form.username, form.password)
      if (success) {
        ElMessage.success('登录成功')
        // 根据角色跳转
        if (authStore.isAdmin) {
          router.push('/admin')
        } else {
          router.push('/')
        }
      }
    } catch (error: any) {
      console.error('登录失败:', error)
      ElMessage.error(error.message || '登录失败，请检查用户名或密码')
    } finally {
      // ⚠️ 关键：无论成功、失败还是抛出异常，都必须关闭 loading
      loading.value = false 
    }
  })
}
</script>

<style scoped>
.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-card {
  width: 400px;
  border-radius: 8px;
}

.login-title {
  text-align: center;
  margin: 0;
  color: #409eff;
  font-size: 28px;
  font-weight: bold;
}

.login-subtitle {
  text-align: center;
  margin: 8px 0 0;
  color: #909399;
  font-size: 14px;
}

.login-button {
  width: 100%;
}

.login-error {
  margin-top: 16px;
}
</style>
