<script setup>
import { reactive } from "vue";
import { IconDownload } from "@tabler/icons-vue";

const prop = defineProps(["item-data"]);
const emit = defineEmits(["downloadRequested"])

const data = reactive({ ...prop.itemData });

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
  <tr>
    <td><!-- Empty cell reserved for table filter --></td>
    <td class="img-container" @click="scaleImage(); emitDownloadRequested()">
      <IconDownload size="30" class="icon-download"/>
      <div class="img-darken" :style="imgDarkenStyle">
        <img :src="prop.itemData.thumbnail">
      </div>
    </td>
    <td class="text-pad">{{ prop.itemData.title }}</td>
    <td class="text-pad">{{ prop.itemData.album_title }}</td>
    <td class="text-pad">{{ prop.itemData.performer_name }}</td>
    <td>{{ Math.floor(prop.itemData.duration / 60) }}:{{ (prop.itemData.duration % 60).toString().padStart(2, "0") }}</td>
  </tr>
</template>

<style scoped>
tr {
  transition: background-color 0.2s ease-in-out;
}

tr:hover {
  background-color: var(--color-hover);
}

.img-container {
  width: 5%;
  margin-left: 20px;
  margin-right: 20px;
  margin-top: auto;
  margin-bottom: auto;
  padding-top: 5px;
  padding-bottom: 5px;
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

td {
  max-width: 100px;
  overflow: scroll;
  white-space: nowrap;
  -ms-overflow-style: none;
}

td::-webkit-scrollbar {
  display: none;
}

.text-pad {
  padding-left: 10px;
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
</style>
