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

    <ul>
      <li v-for="(c, index) in containers" :key="index">{{c.name}}</li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, ref} from "vue";
import { Icon } from "@iconify/vue";
import CreateContainerModal from "../components/CreateContainerModal.vue";
import {useContainerStore} from "../stores";
import {connectWebSocket} from "../api/ws.ts";

const containersStore = useContainerStore();

const containers = computed(() => Object.values(containersStore.containers));

const showCreate = ref(false);


onMounted(() => {
  connectWebSocket((ev) => {
    containersStore.handleEvent(ev);
  })
})
</script>
