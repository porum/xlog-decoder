<script setup lang="ts">
import Item from "./Item.vue";

import EmptyView from "./EmptyView.vue";

defineProps({
  paths: {
    type: Array<string>,
    required: true
  }
})

</script>

<template>
  <div ref="list" class="list" v-if="paths && paths.length > 0">
    <template v-for="item in paths" :key="item">
      <item :path="item" />
    </template>
  </div>
  <empty-view v-else />
</template>

<style scoped lang="less">
.list {
  width: 100%;
  display: flex;
  flex-direction: column;
  overflow: auto;
  scroll-behavior: smooth;
}
</style>