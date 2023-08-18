<script setup>
import { ref, reactive } from "vue";
import { invoke } from '@tauri-apps/api/tauri'
import { IconDownload } from "@tabler/icons-vue";
import SlavartDownloadItem from "../components/SlavartDownloadItem.vue";

let items = ref([1,2,3]);

function handleInput(e) {
  invoke("get_slavart_items", { query: `${e.target.value}` }).then((result) => console.log(result))
}
</script>

<template>
  <div class="container">
    <div class="row">
      <IconDownload size="30" style="display: block; margin: auto" />
      <input placeholder="Song name..." @keypress.enter="handleInput" />
    </div>
    <div class="frame" style="flex-grow: 1">
      <div class="row" style="flex-direction: column;">
        <div class="downloaditem" v-for="(item, index) in items" :key="index"></div>
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
}

.downloaditem {
  min-height: 60px;
}

input {
  flex-grow: 1;
  margin-left: 10px;
}
</style>
