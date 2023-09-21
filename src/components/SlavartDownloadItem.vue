<script setup>
import { reactive } from "vue";
import { IconDownload } from "@tabler/icons-vue";

const prop = defineProps(["item-data", "columns"]);
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
    <td v-show="prop.columns.find((i) => i.key === 'title').enabled" class="text-pad">{{ prop.itemData.title }}</td>
    <td v-show="prop.columns.find((i) => i.key === 'album').enabled" class="text-pad">{{ prop.itemData.albumTitle }}</td>
    <td v-show="prop.columns.find((i) => i.key === 'artist').enabled" class="text-pad">{{ prop.itemData.artist }}</td>
    <td v-show="prop.columns.find((i) => i.key === 'genre').enabled" class="text-pad">{{ prop.itemData.genre }}</td>
    <td v-show="prop.columns.find((i) => i.key === 'duration').enabled">{{ Math.floor(prop.itemData.duration / 60) }}:{{ (prop.itemData.duration % 60).toString().padStart(2, "0") }}</td>
    <td v-show="prop.columns.find((i) => i.key === 'date').enabled" class="text-pad">{{ prop.itemData.date }}</td>
    <td v-show="prop.columns.find((i) => i.key === 'composer').enabled" class="text-pad">{{ prop.itemData.composer }}</td>
    <td v-show="prop.columns.find((i) => i.key === 'isrc').enabled" class="text-pad">{{ prop.itemData.isrc }}</td>
    <td v-show="prop.columns.find((i) => i.key === 'copyright').enabled" class="text-pad">{{ prop.itemData.copyright }}</td>
    <td v-show="prop.columns.find((i) => i.key === 'bitDepth').enabled">{{ prop.itemData.bitDepth }}</td>
    <td v-show="prop.columns.find((i) => i.key === 'samplingRate').enabled">{{ prop.itemData.samplingRate }} kHz</td>
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
  top: 28px;
  left: 26px;
  transform: translate(-50%, -50%);
  z-index: 1;
}

.img-container:hover > .icon-download {
  transition: opacity 0.2s ease;
  opacity: 100%;
}
</style>
