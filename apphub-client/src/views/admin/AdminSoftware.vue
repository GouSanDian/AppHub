<template>
  <div class="admin-software">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>软件管理</span>
          <el-button type="primary" @click="handleAdd">新增软件</el-button>
        </div>
      </template>

      <el-table :data="softwareList" v-loading="loading" style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="name" label="名称" />
        <el-table-column prop="version" label="版本" />
        <el-table-column prop="category_name" label="分类" />
        <el-table-column label="平台" width="200">
          <template #default="{ row }">
            <el-tag
              v-for="p in (row.platform || '').split(',')"
              :key="p"
              size="small"
              style="margin-right: 4px"
            >
              {{ p }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="row.status === 1 ? 'success' : 'warning'">
              {{ row.status === 1 ? '上架' : '下架' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="handleEdit(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 上传对话框 -->
    <el-dialog v-model="dialogVisible" title="新增软件" width="600px">
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
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import request from '@/utils/request'
import type { UploadFile, UploadInstance, FormInstance, FormRules } from 'element-plus'

interface Software {
  id: number
  name: string
  version: string
  description?: string
  file_name: string
  file_size: number
  category_id: number
  category_name: string
  platform: string
  status: number
  download_count: number
  created_at: string
}

interface Category {
  id: number
  name: string
}

const softwareList = ref<Software[]>([])
const categories = ref<Category[]>([])
const loading = ref(false)
const dialogVisible = ref(false)
const uploading = ref(false)
const uploadRef = ref<UploadInstance>()
const uploadFormRef = ref<FormInstance>()
const selectedFile = ref<UploadFile | null>(null)

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

onMounted(() => {
  loadSoftware()
  loadCategories()
})

const loadSoftware = async () => {
  loading.value = true
  try {
    const response = await request.get<any, any>('/softwares')
    softwareList.value = response.data?.list || []
  } catch (error) {
    ElMessage.error('加载软件列表失败')
  } finally {
    loading.value = false
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

const handleAdd = () => {
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

const handleEdit = (row: Software) => {
  ElMessage.info(`编辑软件: ${row.name}`)
}

const handleDelete = async (row: Software) => {
  try {
    await ElMessageBox.confirm(`确认要删除软件 ${row.name} 吗？`, '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    })
    await request.delete(`/softwares/${row.id}`)
    ElMessage.success('删除成功')
    await loadSoftware()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
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
</script>

<style scoped>
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
