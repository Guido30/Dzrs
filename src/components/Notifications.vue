<script setup>
import { ref } from "vue";
import { IconLayoutList } from "@tabler/icons-vue";

const hasNotifications = ref(false);

</script>

<template>
    <IconLayoutList class="icon" id="notification-icon"/>
</template>

<style scoped>
/* TODO */
#notification-icon::after {
    content: "";
    /* display: inline-block; */
    width: 10px;
    height: 10px;
    background-color: var(--color-accent);
    border-radius: 50%;
    /* float: right; */
    position: relative;
    right: 10px;
}

</style>
