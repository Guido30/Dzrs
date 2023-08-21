<script setup>
import { ref, reactive, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri"
import { IconSearch, IconFolder, IconTrash } from "@tabler/icons-vue";
import SlavartDownloadItem from "../components/SlavartDownloadItem.vue";
import HeaderBar from "../components/HeaderBar.vue";
import DownloadInfoItem from "../components/DownloadInfoItem.vue";

const slavartItems = ref([]);
const infoItems = ref([]);

const infoItemsIds = computed({
  get: () => infoItems.value.map((item) => item.id),
  set: (val) => {
    const toRemoveIndex = infoItems.value.findIndex((item) => item.id === val);
    if (toRemoveIndex !== -1) {
      infoItems.value.splice(toRemoveIndex, 1);
    };
  }
});

async function handleInput(e) {
  await invoke("get_slavart_tracks", { query: `${e.target.value}` })
    .then((result) => {slavartItems.value = result.items})
    .catch((err) => console.log("ERR", err));
}

async function downloadTrack(item) {
  if (!infoItemsIds.value.includes(item.id)) {
    infoItems.value.push(item)
  };
  const downloadStatus = await invoke("download_track", { id: item.id, filename: item.title }).then((res) => true).catch((err) => {false; console.log(err);});
};

function removeInfoItem(id) {
  infoItemsIds.value = id
}

</script>

<template>
  <div class="container">
    <HeaderBar :title="'Download'" />
    <div class="row">
      <IconSearch size="30" style="display: block; margin: auto" />
      <input placeholder="Song name..." @keypress.enter="handleInput" />
    </div>
    <div class="row" style="flex-grow: 1; overflow-y: auto; gap: 15px;">
      <div class="frame" style="flex-grow: 1">
        <div class="items-header">
          <p style="width: 5%;"></p>
          <p style="width: 40%;">Title</p>
          <p style="width: 25%;">Album</p>
          <p style="width: 20%;">Artist</p>
          <p style="width: 10%;">Duration</p>
        </div>
        <div class="column">
          <SlavartDownloadItem @downloadRequested="downloadTrack" :item-data="item" v-for="item in slavartItems" :key="item.id"></SlavartDownloadItem>
        </div>
      </div>
      <div class="frame info-items" style="flex-shrink: 0;">
        <div class="items-header">
          <p style="width: 100%;">Downloads</p>
        </div>
        <div>
          <DownloadInfoItem @removeRequested="removeInfoItem" :item-data="item" v-for="item in infoItems" :key="item.id"></DownloadInfoItem>
        </div>
        <div class="row downloads-btns">
            <IconFolder size="30" class="icon"/>
            <IconTrash @click="infoItems = []" size="30" style="cursor: pointer;" class="icon"/>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.frame {
  margin-top: 15px;
  overflow-y: auto;
  overflow-x: hidden;
  padding-left: 10px;
  padding-right: 10px;
  padding-top: 0px;
  padding-bottom: 0px;
}

.items-header {
  display: flex;
  flex-direction: row;
  background-color: #1c1c1c;
  border-bottom: 1px solid #2f2f2f;
  max-height: 30px;
  position: sticky;
  top: 0;
  z-index: 1;
}

.items-header > p {
  font-style: italic;
  margin: 0px;
  margin-top: auto;
  margin-bottom: auto;
  margin-left: 20px;
  margin-right: 20px;
  text-align: center;
}

.info-items {
  width: 200px;
}

.downloads-btns {
  margin-top: auto;
  justify-content: flex-end;
  padding: 5px;
  border-top: 1px solid #2f2f2f;
  position: sticky;
  bottom: 0px;
  background-color: #1c1c1c;
}

input {
  flex-grow: 1;
  margin-left: 10px;
}
</style>
