<template>
  <div class="software">
    <h2 class="page-title">软件中心</h2>

    <el-card>
      <template #header>
        <div class="card-header">
          <div class="header-left">
            <el-input
              v-model="searchQuery"
              placeholder="搜索软件..."
              :prefix-icon="Search"
              clearable
              style="width: 300px"
            />
            <el-select v-model="selectedCategory" placeholder="全部分类" style="width: 150px; margin-left: 16px;">
              <el-option label="全部" value="" />
              <el-option
                v-for="cat in categories"
                :key="cat.id"
                :label="cat.name"
                :value="cat.id"
              />
            </el-select>
          </div>
          <el-button v-if="authStore.isAdmin" type="primary" @click="handleUpload">上传软件</el-button>
        </div>
      </template>

      <el-empty v-if="softwareList.length === 0" description="暂无软件数据" />

      <!-- 软件列表 -->
      <el-row v-else :gutter="16" class="software-list">
        <el-col
          v-for="software in filteredSoftwareList"
          :key="software.id"
          :xs="24"
          :sm="12"
          :md="8"
          :lg="6"
          class="software-col"
        >
          <el-card class="software-card" shadow="hover">
            <div class="software-icon">
              <el-icon :size="48"><Box /></el-icon>
            </div>
            <div class="software-info">
              <h4 class="software-name">{{ software.name }}</h4>
              <p class="software-version">v{{ software.version }}</p>
              <p class="software-desc">{{ software.description || '暂无描述' }}</p>
              <div class="platform-tags">
                <el-tag
                  v-for="p in (software.platform || '').split(',')"
                  :key="p"
                  size="small"
                  style="margin-right: 4px"
                >
                  {{ p }}
                </el-tag>
              </div>
            </div>
            <div class="software-actions">
              <el-button type="primary" size="small" @click="handleDownload(software)">下载</el-button>
            </div>
          </el-card>
        </el-col>
      </el-row>
    </el-card>

    <!-- 上传对话框 -->
    <el-dialog v-model="dialogVisible" title="上传软件" width="600px">
      <el-form
        ref="uploadFormRef"
        :model="uploadForm"
        :rules="uploadRules"
        label-width="100px"
      >
        <el-form-item label="软件名称" prop="name">
          <el-input v-model="uploadForm.name" placeholder="请输入软件名称" />
        </el-form-item>
        <el-form-item label="版本" prop="version">
          <el-input v-model="uploadForm.version" placeholder="如 1.0.0" />
        </el-form-item>
        <el-form-item label="分类" prop="categoryId">
          <el-select v-model="uploadForm.categoryId" placeholder="选择分类">
            <el-option
              v-for="cat in categories"
              :key="cat.id"
              :label="cat.name"
              :value="cat.id"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="平台" prop="platforms">
          <el-checkbox-group v-model="uploadForm.platforms">
            <el-checkbox label="mac">macOS</el-checkbox>
            <el-checkbox label="linux">Linux</el-checkbox>
            <el-checkbox label="windows">Windows</el-checkbox>
          </el-checkbox-group>
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="uploadForm.description" type="textarea" :rows="3" />
        </el-form-item>
        <el-form-item label="安装包" prop="file">
          <el-upload
            ref="uploadRef"
            :auto-upload="false"
            :limit="1"
            :on-change="handleFileChange"
            :on-exceed="handleExceed"
            accept=".dmg,.pkg,.exe,.msi,.deb,.rpm,.AppImage,.tar.gz,.zip"
          >
            <el-button type="primary">选择文件</el-button>
            <template #tip>
              <div class="el-upload__tip">支持 dmg/pkg/exe/msi/deb/rpm 等格式</div>
            </template>
          </el-upload>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="uploading" @click="submitUpload">上传</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { Search, Box } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import request from '@/utils/request'
import { useAuthStore } from '@/stores/auth'
import { useConfigStore } from '@/stores/config'
import type { UploadFile, UploadInstance, FormInstance, FormRules } from 'element-plus'

interface Software {
  id: number
  name: string
  version: string
  description?: string
  category_id: number
  category_name: string
  platform: string
  status: number
}

interface Category {
  id: number
  name: string
}

const searchQuery = ref('')
const selectedCategory = ref<number | string>('')
const softwareList = ref<Software[]>([])
const categories = ref<Category[]>([])
const dialogVisible = ref(false)
const uploading = ref(false)
const uploadRef = ref<UploadInstance>()
const uploadFormRef = ref<FormInstance>()
const selectedFile = ref<UploadFile | null>(null)
const authStore = useAuthStore()
const configStore = useConfigStore()

const uploadForm = reactive({
  name: '',
  version: '',
  categoryId: null as number | null,
  platforms: ['mac', 'linux', 'windows'] as string[],
  description: '',
})

const uploadRules: FormRules = {
  name: [{ required: true, message: '请输入软件名称', trigger: 'blur' }],
  version: [{ required: true, message: '请输入版本号', trigger: 'blur' }],
  categoryId: [{ required: true, message: '请选择分类', trigger: 'change' }],
  platforms: [
    {
      required: true,
      message: '请选择平台',
      trigger: 'change',
      type: 'array',
      min: 1,
    },
  ],
}

const filteredSoftwareList = computed(() => {
  let list = softwareList.value.filter(s => s.status === 1)
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    list = list.filter(s =>
      s.name.toLowerCase().includes(query) ||
      (s.description && s.description.toLowerCase().includes(query))
    )
  }
  if (selectedCategory.value) {
    list = list.filter(s => s.category_id === selectedCategory.value)
  }
  return list
})

onMounted(() => {
  loadSoftware()
  loadCategories()
})

const loadSoftware = async () => {
  try {
    const response = await request.get<any, any>('/softwares')
    softwareList.value = response.data?.list || []
  } catch (error) {
    ElMessage.error('加载软件列表失败')
  }
}

const loadCategories = async () => {
  try {
    const res = await request.get<any, any>('/categories')
    console.log('Categories response:', res)
    categories.value = res.data?.list || res.data || []
    console.log('Categories loaded:', categories.value)
  } catch (error) {
    console.error('Failed to load categories:', error)
    ElMessage.error('加载分类失败: ' + (error as Error).message)
  }
}

const handleUpload = () => {
  Object.assign(uploadForm, {
    name: '',
    version: '',
    categoryId: null,
    platforms: ['mac', 'linux', 'windows'],
    description: '',
  })
  selectedFile.value = null
  uploadRef.value?.clearFiles()
  dialogVisible.value = true
}

const handleFileChange = (file: UploadFile) => {
  selectedFile.value = file
}

const handleExceed = () => {
  ElMessage.warning('只能上传一个文件')
}

const submitUpload = async () => {
  if (!uploadFormRef.value) return
  const valid = await uploadFormRef.value.validate().catch(() => false)
  if (!valid) return
  if (!selectedFile.value?.raw) {
    ElMessage.warning('请选择安装包文件')
    return
  }

  uploading.value = true
  try {
    const formData = new FormData()
    formData.append('name', uploadForm.name)
    formData.append('version', uploadForm.version)
    formData.append('category_id', String(uploadForm.categoryId))
    formData.append('platform', uploadForm.platforms.join(','))
    formData.append('description', uploadForm.description)
    formData.append('file', selectedFile.value.raw)

    await request.post('/softwares', formData)
    ElMessage.success('上传成功')
    dialogVisible.value = false
    await loadSoftware()
  } catch (e) {
    ElMessage.error('上传失败')
  } finally {
    uploading.value = false
  }
}

const handleDownload = async (software: Software) => {
  try {
    const response = await request.get(`/softwares/${software.id}/download`, {
      responseType: 'blob',
      timeout: 5 * 60 * 1000 // 文件下载超时设为 5 分钟
    })

    // 检查是否是错误响应（服务器返回错误时也可能是 Blob）
    const contentType = response.headers['content-type'] || ''
    if (contentType.includes('application/json')) {
      // 服务器返回了 JSON 错误信息
      const text = await response.data.text()
      const json = JSON.parse(text)
      throw new Error(json.message || '下载失败')
    }

    // 检查 Blob 大小，如果太小可能是错误响应
    if (response.data.size < 100) {
      const text = await response.data.text()
      console.warn('下载响应异常:', text)
      // 尝试解析为 JSON 错误
      try {
        const json = JSON.parse(text)
        throw new Error(json.message || '下载失败')
      } catch (e) {
        // 不是 JSON，继续处理
      }
    }

    // 创建 Blob URL
    const blob = new Blob([response.data])
    const url = window.URL.createObjectURL(blob)

    // 清理文件名：去掉路径分隔符与非法字符，避免浏览器丢弃下载
    const rawName = software.file_name || 'download'
    const safeName = rawName.replace(/[\\/:*?"<>|]/g, '_') || 'download'

    // 创建下载链接
    const link = document.createElement('a')
    link.href = url
    link.download = safeName
    link.style.display = 'none'
    document.body.appendChild(link)
    link.click()

    // 延迟清理：让浏览器有时间把 Blob 写入下载队列
    // 立刻 revokeObjectURL 会导致下载被静默取消（下载管理器里啥也没有）
    setTimeout(() => {
      document.body.removeChild(link)
      window.URL.revokeObjectURL(url)
    }, 1000)

    const downloadPath = configStore.downloadPath || '默认下载目录'
    ElMessage.success(`软件下载完成，请查看 ${downloadPath} 目录`)
  } catch (error) {
    console.error('下载失败:', error)
    const message = error instanceof Error ? error.message : '下载失败，请稍后重试'
    ElMessage.error(message)
  }
}
</script>

<style scoped>
.software {
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

.header-left {
  display: flex;
  align-items: center;
}

.software-list {
  margin-top: 16px;
}

.software-col {
  margin-bottom: 16px;
}

.software-card {
  text-align: center;
  padding: 16px;
}

.software-icon {
  width: 80px;
  height: 80px;
  margin: 0 auto 16px;
  background-color: #f5f7fa;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #409eff;
}

.software-name {
  font-size: 16px;
  font-weight: bold;
  color: #303133;
  margin-bottom: 8px;
}

.software-version {
  font-size: 13px;
  color: #909399;
  margin-bottom: 8px;
}

.software-desc {
  font-size: 13px;
  color: #606266;
  margin-bottom: 16px;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.software-actions {
  display: flex;
  justify-content: center;
}
</style>
