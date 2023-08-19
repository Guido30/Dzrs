<script setup>
import { reactive } from "vue";

const prop = defineProps(['item-data']);

</script>

<template>
  <div class="item-data">
    <div style="width: 5%;">
      <img :src="prop.itemData.thumbnail">
    </div>
    <p style="width: 40%;">{{ prop.itemData.title }}</p>
    <p style="width: 25%;">{{ prop.itemData.album_title }}</p>
    <p style="width: 20%;">{{ prop.itemData.performer_name }}</p>
    <p style="width: 10%;">{{ Math.floor(prop.itemData.duration / 60) }}:{{ (prop.itemData.duration % 60).toString().padStart(2, "0") }}</p>
  </div>
</template>

<style scoped>
.item-data {
  display: flex;
  border-radius: 5px;
  min-height: 60px;
}

p {
  margin: 0px;
  margin-top: auto;
  margin-bottom: auto;
  overflow: hidden;
  white-space: nowrap;
}
</style>
