<template>
  <div class="flex min-h-screen items-center justify-center">
    <div class="text-center">
      <h1 class="mb-6 text-3xl font-bold">Minecarts</h1>
      <p class="mb-8 text-gray-400">Create Minecraft servers on Railway.</p>
      <a
        :href="loginUrl"
        class="inline-flex items-center gap-2 rounded-lg bg-blue-600 px-6 py-3 text-white hover:bg-blue-700"
      >
        <Icon icon="carbon:login" width="20" height="20" />
        Login with Railway
      </a>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Icon } from "@iconify/vue";
import { loginUrl } from "../api/auth";
</script>
