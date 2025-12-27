<route lang="yaml">
meta:
  layout: default
</route>

<script setup lang="ts">
import ProfileForm from './components/profile-form.vue'
import SettingsLayout from './components/settings-layout.vue'
</script>

<template>
  <SettingsLayout>
    <ProfileForm />
  </SettingsLayout>
</template>
