<script setup lang="ts">
const route = useRoute()
const router = useRouter()

router.replace({
  path: '/login',
  query: route.query,
})
</script>

<template>
  <div class="min-h-screen bg-background" />
</template>

<route lang="yaml">
meta:
  layout: false
</route>
