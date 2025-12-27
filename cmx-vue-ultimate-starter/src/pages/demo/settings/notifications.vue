<route lang="yaml">
meta:
  layout: default
</route>

<script setup lang="ts">
import NotificationsForm from './components/notifications-form.vue'
import SettingsLayout from './components/settings-layout.vue'
</script>

<template>
  <SettingsLayout>
    <NotificationsForm />
  </SettingsLayout>
</template>
