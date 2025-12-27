<script lang="ts" setup></script>

<template>
  <UiButton
    variant="link"
    class="text-muted-foreground hover:text-primary hover:ring-2 hover:ring-primary"
    @click="$router.push('/auth/forgot-password')"
  >
    忘记密码?
  </UiButton>
</template>

<style scoped></style>
