<template>
  <div class="space-y-6">
    <!-- 诗词统计 -->
    <div class="text-center text-sm text-muted-foreground">
      共找到 {{ filteredPoems.length }} 首诗词
    </div>

    <!-- 诗词列表 -->
    <div class="grid gap-4 max-w-2xl mx-auto">
      <UiCard
        v-for="poem in filteredPoems"
        :key="poem.id"
        class="hover:shadow-lg transition-all duration-300 border-primary/10 relative gap-2"
      >
        <!-- 操作按钮 - 绝对定位在右上角 -->
        <div class="absolute top-3 right-3 flex gap-1 z-10">
          <!-- 朗读按钮 -->
          <UiTooltipProvider>
            <UiTooltip>
              <UiTooltipTrigger as-child>
                <UiButton variant="ghost" size="sm" class="h-8 w-8 p-0" @click="speakPoem(poem)">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="size-4"
                  >
                    <path d="M12 3v12" />
                    <path d="m8 11 4 4 4-4" />
                    <path
                      d="M8 5H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-4"
                    />
                  </svg>
                </UiButton>
              </UiTooltipTrigger>
              <UiTooltipContent>
                <p>朗读</p>
              </UiTooltipContent>
            </UiTooltip>
          </UiTooltipProvider>

          <!-- 解析按钮 -->
          <UiTooltipProvider>
            <UiTooltip>
              <UiTooltipTrigger as-child>
                <UiButton variant="ghost" size="sm" class="h-8 w-8 p-0" @click="showAnalysis(poem)">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="size-4"
                  >
                    <circle cx="12" cy="12" r="10" />
                    <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
                    <path d="M12 17h.01" />
                  </svg>
                </UiButton>
              </UiTooltipTrigger>
              <UiTooltipContent>
                <p>解析</p>
              </UiTooltipContent>
            </UiTooltip>
          </UiTooltipProvider>

          <!-- 收藏按钮 -->
          <UiTooltipProvider>
            <UiTooltip>
              <UiTooltipTrigger as-child>
                <UiButton
                  variant="ghost"
                  size="sm"
                  class="h-8 w-8 p-0"
                  @click="addToCollection(poem)"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="size-4"
                  >
                    <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
                  </svg>
                </UiButton>
              </UiTooltipTrigger>
              <UiTooltipContent>
                <p>收藏</p>
              </UiTooltipContent>
            </UiTooltip>
          </UiTooltipProvider>

          <!-- 分享按钮 -->
          <UiTooltipProvider>
            <UiTooltip>
              <UiTooltipTrigger as-child>
                <UiButton variant="ghost" size="sm" class="h-8 w-8 p-0" @click="sharePoem(poem)">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="size-4"
                  >
                    <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" />
                    <polyline points="16 6 12 2 8 6" />
                    <line x1="12" y1="2" x2="12" y2="15" />
                  </svg>
                </UiButton>
              </UiTooltipTrigger>
              <UiTooltipContent>
                <p>分享</p>
              </UiTooltipContent>
            </UiTooltip>
          </UiTooltipProvider>
        </div>

        <UiCardHeader class="pb-2">
          <div class="text-center">
            <UiCardTitle class="text-xl font-serif text-primary">
              {{ poem.title }}
            </UiCardTitle>
            <UiCardDescription class="text-base">
              {{ poem.author }} · {{ getDynastyName(poem.dynasty) }}
            </UiCardDescription>
          </div>
        </UiCardHeader>

        <UiCardContent class="pt-0">
          <!-- 诗词内容 -->
          <div class="space-y-2 mb-3 text-center">
            <div
              v-for="(line, index) in poem.content"
              :key="index"
              class="text-lg leading-relaxed font-serif text-foreground"
            >
              {{ line }}
            </div>
          </div>

          <!-- 标签 -->
          <div class="flex flex-wrap gap-1 justify-center border-t pt-3">
            <UiBadge v-for="tag in poem.tags" :key="tag" variant="outline" class="text-xs">
              {{ tag }}
            </UiBadge>
          </div>
        </UiCardContent>
      </UiCard>
    </div>

    <!-- 无结果提示 -->
    <div v-if="filteredPoems.length === 0" class="text-center py-12 text-muted-foreground">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="size-16 mx-auto mb-6 text-muted-foreground/50"
      >
        <path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2Z" />
        <path d="m9 10 2 2 4-4" />
      </svg>
      <p class="text-lg">未找到匹配的诗词</p>
      <p class="text-sm mt-2">尝试调整搜索条件或选择其他朝代</p>
    </div>
  </div>
</template>

<script lang="ts" setup>
interface Poetry {
  id: number
  title: string
  author: string
  dynasty: string
  content: string[]
  tags: string[]
}

interface Props {
  searchQuery: string
  selectedDynasty: string
}

const props = defineProps<Props>()

const poems = ref<Poetry[]>([
  {
    id: 1,
    title: '静夜思',
    author: '李白',
    dynasty: 'tang',
    content: ['床前明月光，', '疑是地上霜。', '举头望明月，', '低头思故乡。'],
    tags: ['思乡', '明月', '夜晚'],
  },
  {
    id: 2,
    title: '春晓',
    author: '孟浩然',
    dynasty: 'tang',
    content: ['春眠不觉晓，', '处处闻啼鸟。', '夜来风雨声，', '花落知多少。'],
    tags: ['春天', '自然', '鸟鸣'],
  },
  {
    id: 3,
    title: '水调歌头·明月几时有',
    author: '苏轼',
    dynasty: 'song',
    content: [
      '明月几时有？把酒问青天。',
      '不知天上宫阙，今夕是何年。',
      '我欲乘风归去，又恐琼楼玉宇，高处不胜寒。',
      '起舞弄清影，何似在人间。',
    ],
    tags: ['中秋', '明月', '思念'],
  },
  {
    id: 4,
    title: '登鹳雀楼',
    author: '王之涣',
    dynasty: 'tang',
    content: ['白日依山尽，', '黄河入海流。', '欲穷千里目，', '更上一层楼。'],
    tags: ['登高', '壮丽', '励志'],
  },
  {
    id: 5,
    title: '青玉案·元夕',
    author: '辛弃疾',
    dynasty: 'song',
    content: [
      '东风夜放花千树，更吹落、星如雨。',
      '宝马雕车香满路。',
      '凤箫声动，玉壶光转，一夜鱼龙舞。',
    ],
    tags: ['元宵', '热闹', '繁华'],
  },
])

const filteredPoems = computed(() => {
  return poems.value.filter((poem) => {
    const matchesSearch =
      !props.searchQuery ||
      poem.title.includes(props.searchQuery) ||
      poem.author.includes(props.searchQuery) ||
      poem.content.some((line) => line.includes(props.searchQuery))

    const matchesDynasty = props.selectedDynasty === 'all' || poem.dynasty === props.selectedDynasty

    return matchesSearch && matchesDynasty
  })
})

const getDynastyName = (dynasty: string) => {
  const dynastyMap: Record<string, string> = {
    tang: '唐代',
    song: '宋代',
    yuan: '元代',
    ming: '明代',
    qing: '清代',
  }
  return dynastyMap[dynasty] || dynasty
}

// 朗读诗词
const speakPoem = (poem: Poetry) => {
  const text = `${poem.title}，作者${poem.author}。${poem.content.join(' ')}`
  if ('speechSynthesis' in window) {
    const utterance = new SpeechSynthesisUtterance(text)
    utterance.lang = 'zh-CN'
    utterance.rate = 0.8
    speechSynthesis.speak(utterance)
  } else {
    alert('您的浏览器不支持语音朗读功能')
  }
}

// 显示诗词解析
const showAnalysis = (poem: Poetry) => {
  const analysisMap: Record<number, string> = {
    1: '《静夜思》是唐代诗人李白的代表作之一，表达了诗人对故乡的深切思念。',
    2: '《春晓》描绘了春天早晨的美丽景色，展现了诗人对自然的热爱。',
    3: '《水调歌头》是苏轼的中秋词作，表达了对人生的思考和对亲人的思念。',
    4: '《登鹳雀楼》描绘了壮丽的自然景色，体现了诗人积极向上的精神。',
    5: '《青玉案·元夕》描绘了元宵节的热闹景象，展现了节日的欢乐氛围。',
  }
  alert(analysisMap[poem.id] || '暂无解析')
}

// 收藏诗词
const addToCollection = (poem: Poetry) => {
  const collections = JSON.parse(localStorage.getItem('poetryCollections') || '[]')
  if (!collections.find((item: Poetry) => item.id === poem.id)) {
    collections.push(poem)
    localStorage.setItem('poetryCollections', JSON.stringify(collections))
    alert(`《${poem.title}》已添加到收藏`)
  } else {
    alert(`《${poem.title}》已在收藏中`)
  }
}

// 分享诗词
const sharePoem = (poem: Poetry) => {
  const text = `${poem.title}\n作者：${poem.author}\n${poem.content.join('\n')}`
  if (navigator.share) {
    navigator.share({
      title: poem.title,
      text: text,
    })
  } else if (navigator.clipboard) {
    navigator.clipboard.writeText(text)
    alert('诗词内容已复制到剪贴板')
  } else {
    alert(`分享内容：\n${text}`)
  }
}
</script>

<style scoped></style>
