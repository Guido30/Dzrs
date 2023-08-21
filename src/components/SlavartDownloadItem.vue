<script setup>
import { reactive } from "vue";
import { IconDownload } from "@tabler/icons-vue";

const prop = defineProps(["item-data"]);
const emit = defineEmits(["downloadRequested"])

const data = {
    thumbnail: prop.itemData.thumbnail,
    large: prop.itemData.large,
    performer_name: prop.itemData.performer_name,
    album_title: prop.itemData.album_title,
    duration: prop.itemData.duration,
    title: prop.itemData.title,
    id: prop.itemData.id,
  };

const imgDarkenStyle = reactive({ transform: "scale(1.0)" });

function scaleImage() {
  imgDarkenStyle.transform = "scale(1.1)";
  setTimeout(() => {
    imgDarkenStyle.transform = "scale(1.0)";
  }, 100);
};

function emitDownloadRequested() {
  emit('downloadRequested', data);
};

</script>

<template>
  <div class="item-data">
    <div class="img-container" @click="scaleImage(); emitDownloadRequested()">
      <IconDownload size="30" class="icon-download"/>
      <div class="img-darken" :style="imgDarkenStyle">
        <img :src="prop.itemData.thumbnail">
      </div>
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
  border-radius: 20px;
  min-height: 60px;
}

.item-data:hover {
  transition: background-color 0.2s ease-in-out;
  background-color: #0f0f0f99;
}

.img-container {
  width: 5%;
  margin-left: 20px;
  margin-right: 20px;
  margin-top: auto;
  margin-bottom: auto;
  position: relative;
  cursor: pointer;
  user-select: none;
}

.img-darken {
  opacity: 100%;
}

.img-container:hover > .img-darken {
  transition: opacity 0.2s ease;
  opacity: 50%;
}

img {
  display: block;
  border-radius: 8px;
}

p {
  margin: 0px;
  margin-top: auto;
  margin-bottom: auto;
  margin-left: 20px;
  margin-right: 20px;
  overflow: scroll;
  white-space: nowrap;
  -ms-overflow-style: none;
}

.icon-download {
  opacity: 0%;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 1;
}

.img-container:hover > .icon-download {
  transition: opacity 0.2s ease;
  opacity: 100%;
}

p::-webkit-scrollbar {
  display: none;
}
</style>
