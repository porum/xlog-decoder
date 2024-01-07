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
    <div class="content">
      <!--error: paths.toReversed() is not a function. even if add `ESNext.Array` on tsconfig.json      -->
      <list-view :paths="paths.slice().reverse()" />
    </div>
    <bottom-view @open-preference="openPreference" />
    <preference :show="showPreference" @done="closePreference" />
  </div>
</template>

<style scoped lang="less">
.container {
  background-image: linear-gradient(to bottom, #4EA6D2, #3777B6);
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.content {
  height: calc(100% - 48px);
  display: flex;
  flex: 1;
  justify-content: center;
}
</style>
