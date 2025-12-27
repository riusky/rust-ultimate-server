<route lang="yaml">
meta:
  layout: default
</route>

<template>
  <Page title="代码示例" description="查看和学习各种功能的代码实现示例">
    <div class="max-w-7xl mx-auto space-y-8">
      <!-- 页面标题 -->
      <div class="text-center space-y-4">
        <h1 class="text-3xl font-bold tracking-tight">代码示例</h1>
        <p class="text-lg text-muted-foreground max-w-2xl mx-auto">
          学习各种功能的实现代码，快速应用到您的项目中
        </p>
      </div>

      <!-- 示例分类 -->
      <div class="space-y-6">
        <h2 class="text-2xl font-bold">示例分类</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <UiCard
            v-for="category in categories"
            :key="category.id"
            class="cursor-pointer hover:shadow-md transition-shadow"
            @click="selectedCategory = category.id"
          >
            <UiCardContent class="p-6">
              <div class="flex items-center gap-4">
                <div class="w-12 h-12 rounded-lg bg-primary/10 flex items-center justify-center">
                  <component :is="category.icon" class="w-6 h-6 text-primary" />
                </div>
                <div>
                  <h3 class="font-semibold">{{ category.title }}</h3>
                  <p class="text-sm text-muted-foreground mt-1">{{ category.description }}</p>
                </div>
              </div>
            </UiCardContent>
          </UiCard>
        </div>
      </div>

      <!-- 代码示例列表 -->
      <div class="space-y-6">
        <div class="flex items-center justify-between">
          <h2 class="text-2xl font-bold">代码示例</h2>
          <div class="flex items-center gap-2">
            <UiSelect v-model="difficultyFilter">
              <UiSelectTrigger class="w-32">
                <UiSelectValue placeholder="难度筛选" />
              </UiSelectTrigger>
              <UiSelectContent>
                <UiSelectItem value="all">全部难度</UiSelectItem>
                <UiSelectItem value="beginner">初级</UiSelectItem>
                <UiSelectItem value="intermediate">中级</UiSelectItem>
                <UiSelectItem value="advanced">高级</UiSelectItem>
              </UiSelectContent>
            </UiSelect>
          </div>
        </div>

        <!-- 示例列表 -->
        <div class="grid grid-cols-1 gap-6">
          <UiCard
            v-for="example in filteredExamples"
            :key="example.id"
            class="hover:shadow-md transition-shadow"
          >
            <UiCardHeader>
              <div class="flex items-center justify-between">
                <div>
                  <UiCardTitle>{{ example.title }}</UiCardTitle>
                  <UiCardDescription class="mt-1">
                    {{ example.description }}
                  </UiCardDescription>
                </div>
                <UiBadge :variant="getDifficultyVariant(example.difficulty)">
                  {{ getDifficultyLabel(example.difficulty) }}
                </UiBadge>
              </div>
            </UiCardHeader>
            <UiCardContent>
              <div class="space-y-4">
                <!-- 代码预览 -->
                <div>
                  <h4 class="text-sm font-medium mb-2">代码预览</h4>
                  <div class="bg-muted rounded-lg p-4 font-mono text-sm overflow-x-auto">
                    <pre>{{ example.codePreview }}</pre>
                  </div>
                </div>

                <!-- 使用说明 -->
                <div>
                  <h4 class="text-sm font-medium mb-2">使用说明</h4>
                  <ul class="space-y-1 text-sm text-muted-foreground">
                    <li v-for="(usage, index) in example.usage" :key="index">• {{ usage }}</li>
                  </ul>
                </div>
              </div>
            </UiCardContent>
            <UiCardFooter class="flex justify-between">
              <div class="text-sm text-muted-foreground">
                {{ example.language }}
              </div>
              <UiButton @click="copyCode(example.code)"> 复制代码 </UiButton>
            </UiCardFooter>
          </UiCard>
        </div>

        <!-- 空状态 -->
        <div v-if="filteredExamples.length === 0" class="text-center py-12">
          <Code class="w-16 h-16 text-muted-foreground mx-auto mb-4" />
          <h3 class="text-lg font-semibold mb-2">未找到示例</h3>
          <p class="text-muted-foreground">尝试调整筛选条件</p>
        </div>
      </div>
    </div>
  </Page>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Code, Layout, FormInput, Table, ChartBar, Upload } from 'lucide-vue-next'

// 分类数据
const categories = ref([
  {
    id: 'all',
    title: '全部示例',
    description: '查看所有代码示例',
    icon: Code,
  },
  {
    id: 'ui',
    title: 'UI组件',
    description: '界面组件和布局',
    icon: Layout,
  },
  {
    id: 'forms',
    title: '表单处理',
    description: '表单验证和交互',
    icon: FormInput,
  },
  {
    id: 'data',
    title: '数据处理',
    description: '表格和数据处理',
    icon: Table,
  },
  {
    id: 'charts',
    title: '图表展示',
    description: '数据可视化图表',
    icon: ChartBar,
  },
  {
    id: 'upload',
    title: '文件上传',
    description: '文件上传和处理',
    icon: Upload,
  },
])

// 代码示例数据
const examples = [
  {
    id: 1,
    category: 'ui',
    title: '响应式表格组件',
    description: '一个功能完整的响应式数据表格组件，支持排序、筛选和分页',
    difficulty: 'intermediate',
    language: 'Vue 3 + TypeScript',
    codePreview: `// 表格组件示例代码
const columns = [
  { key: 'name', label: '姓名' },
  { key: 'email', label: '邮箱' },
  { key: 'status', label: '状态' }
]`,
    code: `// 完整的表格组件代码
<template>
  <div class="w-full">
    <div class="flex items-center justify-between mb-4">
      <UiInput
        v-model="searchQuery"
        placeholder="搜索..."
        class="w-64"
      />
    </div>
    <UiTable>
      <UiTableHeader>
        <UiTableRow>
          <UiTableHead v-for="column in columns" :key="column.key">
            {{ column.label }}
          </UiTableHead>
        </UiTableRow>
      </UiTableHeader>
      <UiTableBody>
        <UiTableRow v-for="item in filteredData" :key="item.id">
          <UiTableCell>{{ item.name }}</UiTableCell>
          <UiTableCell>{{ item.email }}</UiTableCell>
          <UiTableCell>
            <UiBadge :variant="item.status === 'active' ? 'default' : 'secondary'">
              {{ item.status }}
            </UiBadge>
          </UiTableCell>
        </UiTableRow>
      </UiTableBody>
    </UiTable>
  </div>
</template>`,
    usage: [
      '在模板中使用 UiTable 组件构建表格结构',
      '通过 computed 属性实现搜索过滤功能',
      '使用 ref 管理表格数据和状态',
      '添加分页控件实现数据分页显示',
    ],
  },
  {
    id: 2,
    category: 'forms',
    title: '表单验证示例',
    description: '使用 Zod 进行表单数据验证的完整示例',
    difficulty: 'beginner',
    language: 'Vue 3 + Zod',
    codePreview: `// 表单验证示例
const schema = z.object({
  name: z.string().min(2),
  email: z.string().email()
})`,
    code: `// 完整的表单验证代码
<template>
  <form @submit.prevent="handleSubmit">
    <div class="space-y-4">
      <UiInput
        v-model="form.name"
        placeholder="姓名"
        :error="errors.name"
      />
      <UiInput
        v-model="form.email"
        placeholder="邮箱"
        :error="errors.email"
      />
      <UiButton type="submit" :disabled="!isValid">
        提交
      </UiButton>
    </div>
  </form>
</template>`,
    usage: [
      '使用 Zod 定义数据验证规则',
      '通过 reactive 创建响应式表单数据',
      '实现实时表单验证反馈',
      '处理表单提交和错误显示',
    ],
  },
  {
    id: 3,
    category: 'data',
    title: '数据卡片展示',
    description: '使用网格布局展示数据卡片的响应式设计',
    difficulty: 'beginner',
    language: 'Vue 3 + Tailwind CSS',
    codePreview: `// 数据卡片网格布局
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
  <UiCard v-for="item in items" :key="item.id">
    <!-- 卡片内容 -->
  </UiCard>
</div>`,
    code: `// 完整的数据卡片代码
<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <UiCard
      v-for="item in dataItems"
      :key="item.id"
      class="hover:shadow-md transition-shadow"
    >
      <UiCardHeader>
        <UiCardTitle>{{ item.title }}</UiCardTitle>
        <UiCardDescription>{{ item.description }}</UiCardDescription>
      </UiCardHeader>
      <UiCardContent>
        <div class="text-2xl font-bold">{{ item.value }}</div>
        <p class="text-sm text-muted-foreground">{{ item.change }}</p>
      </UiCardContent>
    </UiCard>
  </div>
</template>`,
    usage: [
      '使用 grid 布局实现响应式卡片排列',
      '通过 v-for 指令循环渲染卡片',
      '使用条件渲染显示不同状态',
      '添加悬停效果提升用户体验',
    ],
  },
]

// 筛选状态
const selectedCategory = ref('all')
const difficultyFilter = ref('all')

// 计算属性
const filteredExamples = computed(() => {
  return examples.filter((example) => {
    const categoryMatch =
      selectedCategory.value === 'all' || example.category === selectedCategory.value
    const difficultyMatch =
      difficultyFilter.value === 'all' || example.difficulty === difficultyFilter.value
    return categoryMatch && difficultyMatch
  })
})

// 工具函数
const getDifficultyLabel = (difficulty: string) => {
  const labels: Record<string, string> = {
    beginner: '初级',
    intermediate: '中级',
    advanced: '高级',
  }
  return labels[difficulty] || difficulty
}

const getDifficultyVariant = (difficulty: string) => {
  const variants: Record<string, string> = {
    beginner: 'default',
    intermediate: 'secondary',
    advanced: 'destructive',
  }
  return variants[difficulty] as 'default' | 'secondary' | 'destructive' | 'outline'
}

// 复制代码功能
const copyCode = async (code: string) => {
  try {
    await navigator.clipboard.writeText(code)
    // 这里可以添加复制成功的提示
    console.log('代码已复制到剪贴板')
  } catch (err) {
    console.error('复制失败:', err)
  }
}
</script>

<style scoped></style>
