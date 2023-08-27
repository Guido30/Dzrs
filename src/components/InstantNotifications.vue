<script setup>
import { ref, onMounted } from "vue";

import { globalEmitter } from "../helpers";

const notifications = ref([])

function removeNotification() {
    setTimeout(() => {
        notifications.value.splice(0, 1);
    }, 5500);
};

onMounted(() => {
    globalEmitter.on("instant-notification-add", (item) => {
        notifications.value.push(item);
        removeNotification();
    });
});
</script>

<template>
    <div class="column">
        <div class="row" v-for="(item, index) in notifications" :key="index" :class="{ 'info': item.type === 'Info', 'error': item.type === 'Error' }">
            <p>{{ item.msg }}</p>
        </div>
    </div>
</template>

<style scoped>
.column {
    font-size: 0.8em;
    position: absolute;
    bottom: 0px;
    left: 0px;
    justify-content: start;
    margin: 20px;
    z-index: 3;
}

.row {
    opacity: 0;
    max-width: 30vw;
    min-height: 40px;
    border-bottom-left-radius: 5px;
    border-bottom-right-radius: 5px;
    margin-top: 10px;
    padding-left: 10px;
    padding-right: 10px;
    align-items: center;
    box-shadow: 3px 3px 5px var(--color-shadow);
    animation: hide-notification 5.5s linear 0s 1 normal forwards;
}

p {
    margin-top: 5px;
    margin-bottom: 5px;
}

.info {
    border-top: 1px solid var(--color-info);
    background-color: var(--color-bg-info);
}

.error {
    border-top: 1px solid var(--color-error);
    background-color: var(--color-bg-error);
}

@keyframes hide-notification {
    0% {
        opacity: 0;
    }
    5% {
        opacity: 100;
    }
    90% {
        opacity: 100;
    }
    100% {
        opacity: 0;
    }
}
</style>