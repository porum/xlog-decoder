<script setup lang="ts">

import { invoke } from "@tauri-apps/api";

const emits = defineEmits(["openPreference"]);

const onTapSetting = () => {
  emits("openPreference")
}

const onTapFolder = async () => {
  await invoke("show_in_folder", {
    path: localStorage.getItem("dist"),
    opening: true
  })
}
</script>

<template>
  <div class="bottom">
    <p class="copyright">© Sun Guobao</p>
    <div>
      <img src="@/assets/ic_folder.png" alt="" @click="onTapFolder" />
      <img src="@/assets/ic_settings.png" alt="" @click="onTapSetting" />
    </div>
  </div>
</template>

<style scoped lang="less">

.bottom {
  width: 100%;
  height: 48px;
  display: flex;
  flex-direction: row;
  align-content: center;
  justify-content: space-between;
  align-items: center;
  padding-left: 16px;
  padding-right: 8px;
  box-sizing: border-box;

  .copyright {
    color: white;
    font-size: xx-small;
  }

  img {
    width: 24px;
    height: 24px;
    padding: 0 8px;
    pointer-events: auto;
  }
}

</style>