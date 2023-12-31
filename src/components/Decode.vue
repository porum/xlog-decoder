<script setup lang="ts">
import {watch, ref} from "vue";
import {open} from "@tauri-apps/api/dialog";
import {invoke} from "@tauri-apps/api";

enum FileType {
  FILE = "file",
  DIRECTORY = "directory"
}

interface Option {
  value: FileType,
  label: string
}

const options = ref<Option[]>([
  {value: FileType.FILE, label: "File"},
  {value: FileType.DIRECTORY, label: "Dir"}
]);

const selectedType = ref(FileType.FILE);
const placeholder = ref("");
const input = ref("");
const output = ref("");

watch(selectedType, (curVal) => {
  switch (curVal) {
    case FileType.FILE:
      placeholder.value = "Please select .xlog file.";
      break;
    case FileType.DIRECTORY:
      placeholder.value = "Please select the directory of .xlog files.";
      break;
  }
}, {immediate: true});

async function showOpenDialog() {
  const isDirectory = selectedType.value === FileType.DIRECTORY;
  const selected = await open({
    directory: isDirectory,
    multiple: false,
    filters: isDirectory ? [] : [{
      name: "xlog",
      extensions: ["xlog"]
    }]
  });

  if (Array.isArray(selected)) {
    // user selected multiple directories
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    // user selected a single directory
    input.value = selected;
  }
}

function onSelectTypeChanged(event: Event) {
  const target = event.target as HTMLSelectElement;
  selectedType.value = target.value as FileType;
  input.value = "";
  output.value = "";
}

async function decode() {
  output.value = await invoke("decode", {
    name: input.value,
    isDir: selectedType.value === FileType.DIRECTORY
  });
}

</script>

<template>
  <div class="root">
    <div class="input-container">
      <select id="fileType" v-model="selectedType" v-on:change="onSelectTypeChanged">
        <option v-for="option in options" :key="option.value" :value="option.value">{{ option.label }}</option>
      </select>
      <input class="input" readonly v-model="input" v-on:click="showOpenDialog" :placeholder="placeholder"/>
    </div>

    <button class="decode-btn" :disabled="input.length === 0" v-on:click="decode">Decode</button>

    <p>{{ output }}</p>
  </div>
</template>

<style scoped lang="less">
.root {
  margin-top: 64px;
}

.input-container {
  padding-left: 32px;
  padding-right: 32px;
  position: relative;
  box-sizing: border-box;
  display: flex;
  align-items: center;
}

.input {
  height: 36px;
  width: 100%;
}

select {
  position: absolute;
  top: 0;
  right: 32px;
  bottom: 0;
}

.decode-btn {
  margin-top: 32px;
}
</style>
