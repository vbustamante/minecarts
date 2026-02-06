<template>
  <Header v-if="auth.isAuthenticated" />
  <router-view />
</template>

<script setup lang="ts">
import Header from "./components/Header.vue";
import {useAuthStore, useContainerStore} from "./stores";
import {onMounted} from "vue";

const auth = useAuthStore();
const containerStore = useContainerStore();

onMounted(() => {

})


</script>
