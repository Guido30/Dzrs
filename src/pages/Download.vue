<script setup>
import { ref, reactive } from "vue";
import { invoke } from '@tauri-apps/api/tauri'
import { IconDownload } from "@tabler/icons-vue";
import SlavartDownloadItem from "../components/SlavartDownloadItem.vue";

let items = ref([{}]);

function handleInput(e) {
  invoke("get_slavart_items", { query: `${e.target.value}` }).then((result) => items.value = JSON.parse(result)).catch((err) => console.log(err));
}
</script>

<template>
  <div class="container">
    <div class="row">
      <IconDownload size="30" style="display: block; margin: auto" />
      <input placeholder="Song name..." @keypress.enter="handleInput" />
    </div>
    <div class="frame" style="flex-grow: 1">
      <div class="items-header">
        <p style="width: 5%;"></p>
        <p style="width: 40%;">Title</p>
        <p style="width: 25%;">Album</p>
        <p style="width: 20%;">Artist</p>
        <p style="width: 10%;">Duration</p>
      </div>
      <div class="row" style="flex-direction: column;">
        <SlavartDownloadItem :item-data="item" v-for="(item, index) in items" :key="index"></SlavartDownloadItem>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  height: inherit;
  padding-left: 5px;
  padding-right: 5px;
}

.frame {
  margin-top: 10px;
  overflow-y: auto;
  overflow-x: hidden;
  padding-left: 10px;
  padding-right: 10px;
}

.items-header {
  display: flex;
  flex-direction: row;
  max-height: 30px;
}

.items-header > p {
  margin: 0px;
  margin-top: auto;
  margin-bottom: auto;
  text-align: center;
}

input {
  flex-grow: 1;
  margin-left: 10px;
}
</style>
