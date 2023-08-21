<script setup>
import { ref, reactive } from "vue";
import { IconLoader2, IconAlertCircle, IconCircleCheck, IconTrash } from "@tabler/icons-vue";

const prop = defineProps(["item-data"]);
const emit = defineEmits(["removeRequested"]);

function emitRemoveRequested() {
    emit("removeRequested", prop.itemData.id);
};

</script>

<template>
    <div class="item-data">
        <img :src="prop.itemData.large">
        <p>{{ prop.itemData.title }}</p>
        <p>{{ prop.itemData.album_title }}</p>
        <div class="row">
            <p style="flex-grow: 1;">{{ prop.itemData.performer_name }}</p>
            <IconTrash class="icon" style="cursor: pointer;" @click="emitRemoveRequested" />
            <IconLoader2 class="icon icon-loading" v-if="false"/>
            <IconCircleCheck class="icon" v-else-if="true"/>
            <IconAlertCircle class="icon" color="#ff1000" v-else/>
        </div>
    </div>
</template>

<style scoped>

.item-data {
  display: flex;
  flex-direction: column;
  min-height: 60px;
  padding-top: 10px;
  padding-left: 2px;
  padding-bottom: 5px;
  border-bottom: 1px solid #2f2f2f;
}

p {
    text-align: left;
    margin-top: 2px;
    margin-bottom: 2px;
    overflow: scroll;
    white-space: nowrap;
    -ms-overflow-style: none;
}

p::-webkit-scrollbar { 
    display: none;
}

img {
    border-radius: 10px;
    padding: 5px;
    user-select: none;
}

.icon {
    margin-top: auto;
    margin-bottom: auto;
    flex-shrink: 0;
}

.icon-loading {
    animation: icon-loading-anim 1.8s linear infinite;
}

@keyframes icon-loading-anim {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

</style>