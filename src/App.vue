<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import BottomView from "./components/BottomView.vue";
import ListView from "./components/ListView.vue";
import { ref } from "vue";
import { listen } from "@tauri-apps/api/event"
import Preference from "./components/Preference.vue";

const paths = ref<string[]>([])
const showPreference = ref(false)

listen("tauri://file-drop", event => {
  if (event && event.payload) {
    paths.value.push((event.payload as string[])[0])
  }
})

const openPreference = () => {
  showPreference.value = true
}

const closePreference = () => {
  showPreference.value = false
}

</script>

<template>
  <div class="container">
    <!--error: paths.toReversed() is not a function. even if add `ESNext.Array` on tsconfig.json      -->
    <list-view :paths="paths.slice().reverse()" />
    <bottom-view @open-preference="openPreference" />
    <preference :show="showPreference" @done="closePreference" />
  </div>
</template>

<style scoped lang="less">
.container {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-image: linear-gradient(to bottom, #4EA6D2, #3777B6);
  overflow: hidden;
}
</style>
