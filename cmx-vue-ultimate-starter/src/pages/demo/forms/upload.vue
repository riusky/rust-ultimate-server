<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page
    title="文件上传"
    description="这是一个文件上传示例，支持单文件、多文件上传，包含拖拽上传功能"
  >
    <div class="max-w-4xl mx-auto space-y-6">
      <!-- 单文件上传 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>单文件上传</UiCardTitle>
          <UiCardDescription> 支持选择单个文件上传，显示上传进度和文件信息 </UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              class="border-2 border-dashed border-muted-foreground/25 rounded-lg p-8 text-center hover:border-primary/50 transition-colors cursor-pointer"
              @click="selectSingleFile"
              @drop.prevent="handleSingleDrop"
              @dragover.prevent
            >
              <div class="flex flex-col items-center justify-center gap-2">
                <UiUpload class="w-8 h-8 text-muted-foreground" />
                <div class="space-y-1">
                  <p class="text-sm font-medium">点击选择文件或拖拽文件到此处</p>
                  <p class="text-xs text-muted-foreground">
                    支持 JPG, PNG, PDF, DOC, DOCX 格式，文件大小不超过 10MB
                  </p>
                </div>
              </div>
              <input
                ref="singleFileInput"
                type="file"
                class="hidden"
                @change="handleSingleFileSelect"
              />
            </div>

            <!-- 选中的文件 -->
            <div v-if="singleFile" class="space-y-2">
              <div class="flex items-center justify-between p-3 border rounded-lg">
                <div class="flex items-center gap-3">
                  <UiFileText class="w-5 h-5 text-muted-foreground" />
                  <div>
                    <p class="text-sm font-medium">{{ singleFile.name }}</p>
                    <p class="text-xs text-muted-foreground">
                      {{ formatFileSize(singleFile.size) }}
                    </p>
                  </div>
                </div>
                <UiButton
                  variant="ghost"
                  size="sm"
                  @click="removeSingleFile"
                  class="text-destructive hover:text-destructive"
                >
                  <UiX class="w-4 h-4" />
                </UiButton>
              </div>

              <!-- 上传进度 -->
              <div v-if="singleUploadProgress > 0" class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span>上传进度</span>
                  <span>{{ singleUploadProgress }}%</span>
                </div>
                <UiProgress :value="singleUploadProgress" class="w-full" />
              </div>

              <UiButton @click="uploadSingleFile" :disabled="isUploadingSingle" class="w-full">
                <UiLoader2 v-if="isUploadingSingle" class="w-4 h-4 mr-2 animate-spin" />
                {{ isUploadingSingle ? '上传中...' : '上传文件' }}
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 多文件上传 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>多文件上传</UiCardTitle>
          <UiCardDescription> 支持选择多个文件同时上传，批量管理文件 </UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              class="border-2 border-dashed border-muted-foreground/25 rounded-lg p-8 text-center hover:border-primary/50 transition-colors cursor-pointer"
              @click="selectMultipleFiles"
              @drop.prevent="handleMultipleDrop"
              @dragover.prevent
            >
              <div class="flex flex-col items-center justify-center gap-2">
                <UiUpload class="w-8 h-8 text-muted-foreground" />
                <div class="space-y-1">
                  <p class="text-sm font-medium">点击选择文件或拖拽文件到此处</p>
                  <p class="text-xs text-muted-foreground">
                    支持多种文件格式，最多可上传 10 个文件，每个文件不超过 5MB
                  </p>
                </div>
              </div>
              <input
                ref="multipleFileInput"
                type="file"
                multiple
                class="hidden"
                @change="handleMultipleFileSelect"
              />
            </div>

            <!-- 文件列表 -->
            <div v-if="multipleFiles.length > 0" class="space-y-2">
              <div class="flex justify-between items-center">
                <p class="text-sm font-medium">已选择 {{ multipleFiles.length }} 个文件</p>
                <UiButton
                  variant="ghost"
                  size="sm"
                  @click="clearMultipleFiles"
                  class="text-destructive hover:text-destructive"
                >
                  清空全部
                </UiButton>
              </div>

              <div class="space-y-2 max-h-60 overflow-y-auto">
                <div
                  v-for="(file, index) in multipleFiles"
                  :key="index"
                  class="flex items-center justify-between p-3 border rounded-lg"
                >
                  <div class="flex items-center gap-3">
                    <UiFileText class="w-5 h-5 text-muted-foreground" />
                    <div>
                      <p class="text-sm font-medium">{{ file.name }}</p>
                      <p class="text-xs text-muted-foreground">
                        {{ formatFileSize(file.size) }}
                      </p>
                    </div>
                  </div>
                  <div class="flex items-center gap-2">
                    <div
                      v-if="
                        fileUploadProgress[index] !== undefined && fileUploadProgress[index] > 0
                      "
                      class="w-16 text-xs text-right"
                    >
                      {{ fileUploadProgress[index] }}%
                    </div>
                    <UiButton
                      variant="ghost"
                      size="sm"
                      @click="removeMultipleFile(index)"
                      class="text-destructive hover:text-destructive"
                    >
                      <UiX class="w-4 h-4" />
                    </UiButton>
                  </div>
                </div>
              </div>

              <UiButton @click="uploadMultipleFiles" :disabled="isUploadingMultiple" class="w-full">
                <UiLoader2 v-if="isUploadingMultiple" class="w-4 h-4 mr-2 animate-spin" />
                {{ isUploadingMultiple ? '上传中...' : '上传所有文件' }}
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 图片上传预览 -->
      <UiCard>
        <UiCardHeader>
          <UiCardTitle>图片上传预览</UiCardTitle>
          <UiCardDescription> 支持图片预览、裁剪和压缩功能 </UiCardDescription>
        </UiCardHeader>
        <UiCardContent>
          <div class="space-y-4">
            <div
              class="border-2 border-dashed border-muted-foreground/25 rounded-lg p-8 text-center hover:border-primary/50 transition-colors cursor-pointer"
              @click="selectImageFile"
              @drop.prevent="handleImageDrop"
              @dragover.prevent
            >
              <div class="flex flex-col items-center justify-center gap-2">
                <UiImage class="w-8 h-8 text-muted-foreground" />
                <div class="space-y-1">
                  <p class="text-sm font-medium">点击选择图片或拖拽图片到此处</p>
                  <p class="text-xs text-muted-foreground">
                    支持 JPG, PNG, GIF 格式，建议尺寸 800x600 以上
                  </p>
                </div>
              </div>
              <input
                ref="imageFileInput"
                type="file"
                accept="image/*"
                class="hidden"
                @change="handleImageFileSelect"
              />
            </div>

            <!-- 图片预览 -->
            <div v-if="imageFile" class="space-y-4">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="space-y-2">
                  <p class="text-sm font-medium">原图预览</p>
                  <div class="border rounded-lg p-4">
                    <img
                      :src="imagePreview"
                      :alt="imageFile.name"
                      class="w-full h-auto max-h-48 object-contain"
                    />
                  </div>
                </div>
                <div class="space-y-2">
                  <p class="text-sm font-medium">文件信息</p>
                  <div class="space-y-2 text-sm">
                    <div class="flex justify-between">
                      <span>文件名:</span>
                      <span class="font-medium">{{ imageFile.name }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span>文件大小:</span>
                      <span class="font-medium">{{ formatFileSize(imageFile.size) }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span>文件类型:</span>
                      <span class="font-medium">{{ imageFile.type }}</span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 图片处理选项 -->
              <div class="space-y-3">
                <div class="flex items-center justify-between">
                  <UiLabel for="quality">图片质量</UiLabel>
                  <UiSelect v-model="imageQuality">
                    <UiSelectTrigger class="w-32">
                      <UiSelectValue placeholder="选择质量" />
                    </UiSelectTrigger>
                    <UiSelectContent>
                      <UiSelectItem value="high">高质量</UiSelectItem>
                      <UiSelectItem value="medium">中等质量</UiSelectItem>
                      <UiSelectItem value="low">低质量</UiSelectItem>
                    </UiSelectContent>
                  </UiSelect>
                </div>

                <div class="flex items-center justify-between">
                  <UiLabel for="resize">调整尺寸</UiLabel>
                  <UiSelect v-model="imageResize">
                    <UiSelectTrigger class="w-32">
                      <UiSelectValue placeholder="选择尺寸" />
                    </UiSelectTrigger>
                    <UiSelectContent>
                      <UiSelectItem value="original">原尺寸</UiSelectItem>
                      <UiSelectItem value="large">大尺寸</UiSelectItem>
                      <UiSelectItem value="medium">中尺寸</UiSelectItem>
                      <UiSelectItem value="small">小尺寸</UiSelectItem>
                    </UiSelectContent>
                  </UiSelect>
                </div>
              </div>

              <UiButton @click="uploadImage" :disabled="isUploadingImage" class="w-full">
                <UiLoader2 v-if="isUploadingImage" class="w-4 h-4 mr-2 animate-spin" />
                {{ isUploadingImage ? '上传中...' : '上传图片' }}
              </UiButton>
            </div>
          </div>
        </UiCardContent>
      </UiCard>

      <!-- 上传结果 -->
      <UiAlert
        v-if="uploadResult"
        :variant="uploadResult.type"
        class="animate-in fade-in-0 duration-300"
      >
        <UiCheckCircle v-if="uploadResult.type === 'default'" class="w-4 h-4" />
        <UiAlertCircle v-if="uploadResult.type === 'destructive'" class="w-4 h-4" />
        <UiAlertTitle>{{ uploadResult.title }}</UiAlertTitle>
        <UiAlertDescription>{{ uploadResult.message }}</UiAlertDescription>
      </UiAlert>
    </div>
  </Page>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import Page from '@/components/global-layout/basic-page.vue'

// 单文件上传相关
const singleFileInput = ref<HTMLInputElement>()
const singleFile = ref<File | null>(null)
const singleUploadProgress = ref(0)
const isUploadingSingle = ref(false)

// 多文件上传相关
const multipleFileInput = ref<HTMLInputElement>()
const multipleFiles = ref<File[]>([])
const fileUploadProgress = ref<number[]>([])
const isUploadingMultiple = ref(false)

// 图片上传相关
const imageFileInput = ref<HTMLInputElement>()
const imageFile = ref<File | null>(null)
const imagePreview = ref('')
const imageQuality = ref('medium')
const imageResize = ref('original')
const isUploadingImage = ref(false)

// 上传结果
const uploadResult = ref<{
  type: 'default' | 'destructive'
  title: string
  message: string
} | null>(null)

// 选择单文件
const selectSingleFile = () => {
  singleFileInput.value?.click()
}

// 处理单文件选择
const handleSingleFileSelect = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    singleFile.value = file
    singleUploadProgress.value = 0
  }
}

// 处理单文件拖拽
const handleSingleDrop = (event: DragEvent) => {
  const files = event.dataTransfer?.files
  if (files && files.length > 0) {
    singleFile.value = files[0] as File
    singleUploadProgress.value = 0
  }
}

// 移除单文件
const removeSingleFile = () => {
  singleFile.value = null
  singleUploadProgress.value = 0
  if (singleFileInput.value) {
    singleFileInput.value.value = ''
  }
}

// 选择多文件
const selectMultipleFiles = () => {
  multipleFileInput.value?.click()
}

// 处理多文件选择
const handleMultipleFileSelect = (event: Event) => {
  const target = event.target as HTMLInputElement
  const files = Array.from(target.files || [])
  if (files.length > 0) {
    multipleFiles.value = [
      ...multipleFiles.value,
      ...files.slice(0, 10 - multipleFiles.value.length),
    ]
    fileUploadProgress.value = Array(multipleFiles.value.length).fill(0)
  }
}

// 处理多文件拖拽
const handleMultipleDrop = (event: DragEvent) => {
  const files = Array.from(event.dataTransfer?.files || [])
  if (files.length > 0) {
    multipleFiles.value = [
      ...multipleFiles.value,
      ...files.slice(0, 10 - multipleFiles.value.length),
    ]
    fileUploadProgress.value = Array(multipleFiles.value.length).fill(0)
  }
}

// 移除多文件
const removeMultipleFile = (index: number) => {
  multipleFiles.value.splice(index, 1)
  fileUploadProgress.value.splice(index, 1)
}

// 清空多文件
const clearMultipleFiles = () => {
  multipleFiles.value = []
  fileUploadProgress.value = []
  if (multipleFileInput.value) {
    multipleFileInput.value.value = ''
  }
}

// 选择图片文件
const selectImageFile = () => {
  imageFileInput.value?.click()
}

// 处理图片文件选择
const handleImageFileSelect = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file && file.type.startsWith('image/')) {
    imageFile.value = file
    const reader = new FileReader()
    reader.onload = (e) => {
      imagePreview.value = e.target?.result as string
    }
    reader.readAsDataURL(file)
  }
}

// 处理图片拖拽
const handleImageDrop = (event: DragEvent) => {
  const files = event.dataTransfer?.files
  if (files && files.length > 0) {
    const file = files[0]
    if (file && file.type.startsWith('image/')) {
      imageFile.value = file as File
      const reader = new FileReader()
      reader.onload = (e) => {
        imagePreview.value = e.target?.result as string
      }
      reader.readAsDataURL(file as Blob)
    }
  }
}

// 格式化文件大小
const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// 上传单文件
const uploadSingleFile = async () => {
  if (!singleFile.value) return

  isUploadingSingle.value = true
  singleUploadProgress.value = 0

  try {
    // 模拟上传过程
    for (let i = 0; i <= 100; i += 10) {
      await new Promise((resolve) => setTimeout(resolve, 200))
      singleUploadProgress.value = i
    }

    showUploadResult('default', '上传成功', `文件 "${singleFile.value.name}" 已成功上传`)
    removeSingleFile()
  } catch {
    showUploadResult('destructive', '上传失败', '文件上传过程中出现错误')
  } finally {
    isUploadingSingle.value = false
  }
}

// 上传多文件
const uploadMultipleFiles = async () => {
  if (multipleFiles.value.length === 0) return

  isUploadingMultiple.value = true

  try {
    // 模拟上传过程
    for (let i = 0; i <= 100; i += 10) {
      await new Promise((resolve) => setTimeout(resolve, 100))
      fileUploadProgress.value = fileUploadProgress.value.map(() => i)
    }

    showUploadResult('default', '上传成功', `${multipleFiles.value.length} 个文件已成功上传`)
    clearMultipleFiles()
  } catch {
    showUploadResult('destructive', '上传失败', '文件上传过程中出现错误')
  } finally {
    isUploadingMultiple.value = false
  }
}

// 上传图片
const uploadImage = async () => {
  if (!imageFile.value) return

  isUploadingImage.value = true

  try {
    // 模拟上传过程
    await new Promise((resolve) => setTimeout(resolve, 2000))

    showUploadResult('default', '图片上传成功', `图片 "${imageFile.value.name}" 已成功上传`)
    imageFile.value = null
    imagePreview.value = ''
    if (imageFileInput.value) {
      imageFileInput.value.value = ''
    }
  } catch {
    showUploadResult('destructive', '上传失败', '文件上传过程中出现错误')
  } finally {
    isUploadingImage.value = false
  }
}

// 显示上传结果
const showUploadResult = (type: 'default' | 'destructive', title: string, message: string) => {
  uploadResult.value = { type, title, message }
  setTimeout(() => {
    uploadResult.value = null
  }, 5000)
}
</script>

<style scoped></style>
