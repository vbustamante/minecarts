<template>
  <div>
    <h1>Minecarts</h1>
    <p>Create Minecraft servers on Railway.</p>

    <button
      @click="showCreate = true"
      class="mt-4 rounded-full bg-blue-600 p-2 text-white hover:bg-blue-700"
    >
      <Icon icon="carbon:add" width="24" height="24" />
    </button>

    <CreateContainerModal :open="showCreate" @close="showCreate = false" />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Icon } from "@iconify/vue";
import CreateContainerModal from "../components/CreateContainerModal.vue";

const showCreate = ref(false);
</script>
