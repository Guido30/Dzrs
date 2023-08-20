<script setup>
import router from "../router";
import { IconChevronsLeft } from "@tabler/icons-vue";

let props = defineProps(["title"])
</script>

<template>
  <div id="button" class="row" style="margin-bottom: 10px;" @click="router.push({ path: '/'})">
    <div class="row" style="padding: 10px;">
        <IconChevronsLeft size="25" style="display: block; margin: auto; margin-right: 10px;" />
        <h1>{{ props.title }}</h1>
    </div>
    <div style="flex-grow: 1;"></div>
  </div>
</template>

<style scoped>
h1 {
    font-size: 1.2em;
    font-style: italic;
    margin-right: 10px;
    margin-top: 2px;
    margin-bottom: 2px;
    user-select: none;
}
#button {
    background-color: rgba(15, 15, 15, 0.6);
    border-radius: 40px;
    box-shadow: 0 2px 2px transparent;
    border: 1px solid transparent;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}
#button:hover {
    transition: all 0.2s ease-in-out;
    background-color: rgba(15, 15, 15, 0.3);
}
</style>
