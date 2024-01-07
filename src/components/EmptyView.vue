<script setup lang="ts">

</script>

<template>
  <div class="empty">
    <img src="../assets/ic_empty.png" alt="" />
    <p>Drag and drop the .xlog file here!</p>
  </div>
</template>

<style scoped lang="less">
.empty {
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