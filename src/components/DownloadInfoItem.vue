<script setup>
import { ref } from "vue";
import { IconLoader2, IconAlertCircle, IconCircleCheck, IconTrash } from "@tabler/icons-vue";

const prop = defineProps(["item-data"]);
const emit = defineEmits(["removeRequested"]);

const showStatusMessage = ref(false);

function emitRemoveRequested() {
    emit("removeRequested", prop.itemData.id);
};

</script>

<template>
    <div class="item-data">
        <img :src="prop.itemData.large">
        <p>{{ prop.itemData.title }}</p>
        <p>{{ prop.itemData.albumTitle }}</p>
        <div class="row">
            <p style="flex-grow: 1;">{{ prop.itemData.artist }}</p>
            <IconTrash class="icon icon-trash" style="cursor: pointer; margin-left: 5px;" @click="emitRemoveRequested" />
            <IconLoader2 class="icon icon-loading" v-if="!prop.itemData.hasOwnProperty('status')"/>
            <IconCircleCheck class="icon" color="var(--color-success)" v-else-if="prop.itemData.status"/>
            <IconAlertCircle @click="showStatusMessage = !showStatusMessage" class="icon" style="cursor: pointer;" color="var(--color-error)" v-else/>
        </div>
        <div class="status-menu" v-show="showStatusMessage">
            {{ prop.itemData.statusMsg }}
        </div>
    </div>
</template>

<style scoped>

.item-data {
    font-size: 14px;
    display: flex;
    flex-direction: column;
    min-height: 60px;
    padding-top: 10px;
    padding-left: 2px;
    padding-bottom: 5px;
    border-bottom: 1px solid var(--color-bg-2);
}

p {
    text-align: left;
    margin-top: 2px;
    margin-bottom: 0px;
    margin-right: 2px;
    padding-left: 5px;
    padding-right: 5px;
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

.status-menu {
    text-align: left;
    margin-top: 5px;
    margin-bottom: 5px;
    padding: 5px;
    overflow-wrap: break-word;
    background-color: var(--color-bg-2);
    border: 1px solid var(--color-accent);
    border-radius: 5px;
}

.icon-loading {
    animation: icon-loading-anim 1.8s linear infinite;
}

.icon-trash {
    opacity: 0%;
    transition: opacity 0.2s ease;
}

.item-data:hover .icon-trash {
    opacity: 100%;
}

</style>