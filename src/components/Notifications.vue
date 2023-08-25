<script setup>
import { computed, onMounted, ref } from "vue";
import { globalEmitter } from '../helpers';
import { IconX } from '@tabler/icons-vue';

const notifications = ref([])
const hasNotifications = computed(() => !!notifications.value.length);
const noficationsCount = computed(() => notifications.value.length);

onMounted(() => {
    globalEmitter.on("notification-add", (item) => {
        notifications.value.push(item)
    })
})

</script>

<template>
    <div class="frame column notifications-container">
        <div class="row notifications-header" :class="{ 'notifications-header-border': !hasNotifications }">
            <p style="margin-right: auto; padding-left: 10px;">Notifications</p>
            <div style="width: 34px; margin-top: auto; margin-bottom: auto; padding-right: 10px;">
                <div class="count-wrapper" >
                    <p style="font-style: normal; font-size: 0.8em; margin-top: 5px; margin-bottom: 5px;">{{ noficationsCount }}</p>
                </div>
            </div>
        </div>
        <div class="column notifications-body">
            <div class="notification" v-for="(item, index) in notifications" :key="index">
                <div class="row" style="justify-content: start;">
                    <span class="notification-header">Type: {{ item.type }}</span>
                    <span class="notification-header">Origin: {{ item.origin }}</span>
                    <IconX class="icon icon-x" size="20" color="var(--color-text)" @click="notifications.splice(i, 1)"/>
                </div>
                <p style="overflow-wrap: break-word;">{{ item.msg }}</p>
            </div>
        </div>
    </div>
</template>

<style scoped>

.notifications-container {
    width: 35vw;
    min-height: 90px;
    max-height: calc(100vh - 95px);
    position: absolute;
    top: 75px;
    right: 10px;
    z-index: 5;
    overflow-y: auto;
    border-color: var(--color-accent);
    border-radius: 15px;
    box-shadow: 2px 4px 10px #111111;
}

.notifications-container > div {
    padding-left: 10px;
    padding-right: 10px;
}

.notifications-header {
    font-size: 1.5em;
    font-style: italic;
    justify-content: start;
}

.notifications-header-border {
    border-bottom: 1px solid var(--color-bg-2);
}

.notifications-header p {
    padding-bottom: 5px;
    padding-top: 5px;
}

.notifications-body {
    gap: 10px;
    padding-bottom: 10px;
}

.count-wrapper {
    margin-top: 10px;
    margin-bottom: 10px;
    background-color: var(--color-accent);
    box-shadow: 2px 2px 0px var(--color-shadow);
    border-radius: 50%;
    user-select: none;
}

.notification {
    position: relative;
    min-height: 60px;
    border-top: 1px solid var(--color-bg-2);
    transition: all 0.2s ease;
    padding-left: 10px;
    padding-right: 10px;
}

.notification:hover {
    border-top: 1px solid var(--color-accent);
    background-color: var(--color-bg-2);
}

.notification-header {
    font-style: italic;
    font-weight: 300;
    min-width: 120px;
    text-align: left;
}

.icon-x {
    opacity: 0%;
    transition: opacity 0.2s ease;
    cursor: pointer;
    border-radius: 5px;
    background-color: var(--color-accent);
    margin-left: auto;
}

.notification:hover .icon-x {
    opacity: 100%;
}

</style>
