<script setup lang="ts">
import Item from "./Item.vue";

defineProps({
  paths: {
    type: Array<string>,
    required: true
  }
})

</script>

<template>
  <div v-if="paths && paths.length > 0" ref="list" class="list">
    <item v-for="item in paths" :key="item" :path="item" />
  </div>
  <div v-else class="empty">
    <img src="@/assets/ic_empty.png" alt="" />
    <p>Drag and drop the .xlog file here!</p>
  </div>
</template>

<style scoped lang="less">
.list {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  scroll-behavior: smooth;
}

.empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;

  img {
    width: 80px;
    height: 80px;
  }

  p {
    margin-top: 16px;
    font-weight: bold;
    color: white;
    opacity: 80%;
  }
}
</style>