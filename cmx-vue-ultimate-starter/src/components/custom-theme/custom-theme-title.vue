<script lang="ts" setup>
import { transformI18n } from '@/plugins/i18n'
</script>

<template>
  <div class="grid space-y-1">
    <h1 class="font-semibold text-md text-foreground">
      {{ transformI18n('common.interfaceLabel.customize') }}
    </h1>
    <p class="text-xs text-muted-foreground">
      {{ transformI18n('common.interfaceMessage.themePick') }}
    </p>
  </div>
</template>
