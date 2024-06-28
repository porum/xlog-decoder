<script setup lang="ts">

import { ref, watch } from "vue"
import { basename, join, resolveResource } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api";
import { createDir, exists } from "@tauri-apps/api/fs";

const props = defineProps({
  path: {
    type: String,
    required: true
  }
})

/** resolved decode file path */
const resolvedPath = ref("")
/** decode file name */
const name = ref("")
/** decode dist path */
const dist = ref("")
/** decode result, 0 is success */
const status = ref(-1)

watch(() => props.path, async (newValue, _) => {
  const xlogPath = await resolveResource(newValue!)
  name.value = await basename(xlogPath)
  resolvedPath.value = xlogPath
  const code = await decode()
  if (code !== 0) {
    dist.value = ""
  }
  status.value = code

}, {immediate: true})

const decode = async (): Promise<number> => {
  const customDistPath = localStorage.getItem("dist")
  let distPath: string
  if (!resolvedPath.value.endsWith(".xlog")) { // dir not support custom dist path
    distPath = ""
  } else if (customDistPath) { // custom dist path
    if (!await exists(customDistPath)) {
      await createDir(customDistPath)
    }
    distPath = await join(customDistPath, `${name.value}.log`)
  } else { // default dist path
    distPath = `${props.path}.log`
  }
  // preset, if decode failed, then clear
  dist.value = distPath.length === 0 ? resolvedPath.value : distPath
  return await invoke("decode", {
    name: props.path,
    privateKey: localStorage.getItem("key") ?? "",
    dist: distPath
  })
}

const showInFolder = async () => {
  if (status.value === 0) {
    await invoke("show_in_folder", {
      path: dist.value,
      opening: false,
    })
  }
}

</script>

<template>
  <div class="item" @click="showInFolder">
    <div class="filename">{{ name }}</div>
    <img src="@/assets/ic_success.png" alt="" v-if="status === 0" />
    <img src="@/assets/ic_error.png" alt="" v-else-if="status !== 0 && status !== -1" />
  </div>
</template>

<style scoped lang="less">
.item {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 8px 16px;
  box-sizing: border-box;

  .filename {
    flex: 1;
    overflow-x: hidden;
    color: white;
    font-weight: bold;
    white-space: normal;
    word-wrap: break-word;
    -webkit-touch-callout: none; /* iOS Safari */
    -webkit-user-select: none; /* Safari */
    -khtml-user-select: none; /* Konqueror HTML */
    -moz-user-select: none; /* Firefox */
    -ms-user-select: none; /* Internet Explorer/Edge */
    user-select: none; /* Non-prefixed version, currently supported by Chrome and Opera */
  }

  img {
    width: 32px;
    height: 32px;
    margin-left: 8px;
  }
}
</style>