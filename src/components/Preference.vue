<script setup lang="ts">
import { ref } from "vue";
import { open } from "@tauri-apps/api/dialog";

const privateKey = ref(localStorage.getItem("key") ?? "");
const distPath = ref(localStorage.getItem("dist") ?? "");

const emits = defineEmits(["done"]);

defineProps({
  show: {
    type: Boolean,
    required: false,
    default: false
  }
})

const done = () => {
  localStorage.setItem("key", privateKey.value)
  emits("done")
}

const showOpenDialog = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
  })

  if (Array.isArray(selected)) {
    // user selected multiple directories
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    // user selected a single directory
    distPath.value = selected
    localStorage.setItem("dist", selected)
  }
}

</script>

<template>
  <div v-show="show" class="preference">
    <div class="item" style="margin-bottom: 8px">
      <span>Save To:</span>
      <input readonly v-model="distPath" @click="showOpenDialog" />
      <div v-if="false" class="action-btn" @click="showOpenDialog">…</div>
    </div>
    <div class="item">
      <span>Private Key:</span>
      <input v-model="privateKey" />
      <div class="action-btn" @click="done">✓</div>
    </div>
  </div>
</template>

<style scoped lang="less">
.preference {
  position: fixed;
  bottom: 0;
  padding: 8px;
  background: gainsboro;
  width: 100%;
  height: auto;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
  box-shadow: 0 2px 8px gray inset;
}

.item {
  display: flex;
  flex-direction: row;
  align-items: center;
  font-size: smaller;
  white-space: nowrap;

  span {
    text-align: right;
    width: 72px;
    margin-right: 8px;
    font-weight: 500;
    max-lines: 1;
    overflow: hidden;
  }

  input {
    flex: 1;
    height: 24px;
    background: white;
    outline: none;
    border: 1px;
    box-shadow: none;
    border-radius: 0;
    padding: 0 4px;
    margin: 0;
  }
}

.action-btn {
  margin-left: 8px;
  width: 24px;
  height: 24px;
  background: white;
  text-align: center;
}

@media (prefers-color-scheme: dark) {
  .preference {
    background: #2f2f2f;
    box-shadow: none;
  }

  .item {
    input {
      background: gray;
    }
  }

  .action-btn {
    background: gray;
  }
}

</style>