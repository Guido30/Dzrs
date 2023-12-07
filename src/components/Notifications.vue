<script setup>
import { computed, onMounted, ref, watch } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { IconX, IconTrash } from "@tabler/icons-vue";

const notifications = ref([]);
const notificationsCount = computed(() => notifications.value.length);

onMounted(async () => {
  await appWindow.listen("notification-add", async (item) => {
    await appWindow.emit("instant-notification-add", item.payload);
    notifications.value.push(item.payload);
  });
});

watch(notificationsCount, async () => {
  await appWindow.emit("show-notifications-dot", !!notificationsCount.value);
});
</script>

<template>
  <div class="frame column notifications-container">
    <div class="row notifications-header">
      <p style="margin-right: auto; padding-left: 10px">Notifications</p>
      <div style="width: 34px; margin-top: auto; margin-bottom: auto; padding-right: 10px">
        <div class="count-wrapper">
          <p style="font-style: normal; font-size: 0.8em; margin-top: 5px; margin-bottom: 5px">
            {{ notificationsCount }}
          </p>
        </div>
      </div>
    </div>
    <div class="column notifications-body">
      <div class="column" style="gap: 5px">
        <div class="notification" v-for="(item, index) in notifications" :key="index" :class="{ info: item.type === 'Info', error: item.type === 'Error' }">
          <div class="row" style="justify-content: start">
            <span class="notification-header">From {{ item.origin }}</span>
            <IconX class="icon icon-x" size="20" @click="notifications.splice(i, 1)" />
          </div>
          <p style="overflow-wrap: break-word">{{ item.msg }}</p>
        </div>
      </div>
    </div>
    <div class="row notifications-footer">
      <IconTrash v-tooltip="'Clear Notifications'" size="30" class="icon icon-trash clickable-effect" @click="notifications = []" />
    </div>
  </div>
</template>

<style scoped>
.notifications-container {
  width: 35vw;
  height: calc(100vh - 95px);
  justify-content: start;
  position: absolute;
  top: 75px;
  right: 10px;
  z-index: 5;
  overflow-y: auto;
  border-color: var(--color-accent);
  border-radius: 15px;
  box-shadow: 2px 4px 10px #111111;
  padding-top: 0px;
  padding-bottom: 0px;
  overflow-y: hidden;
}

.notifications-container > div {
  padding-left: 10px;
  padding-right: 10px;
}

.notifications-header {
  font-size: 1.5em;
  font-style: italic;
  justify-content: start;
  position: sticky;
  top: 0px;
  background-color: var(--color-bg-1);
  z-index: 1;
  border-bottom: 1px solid var(--color-bg-2);
}

.notifications-header p {
  padding-bottom: 5px;
  padding-top: 5px;
}

.notifications-body {
  gap: 10px;
  padding-top: 5px;
  padding-bottom: 5px;
  flex-grow: 1;
  justify-content: start;
  overflow-y: auto;
}

.notifications-footer {
  justify-content: end;
  padding-bottom: 10px;
  position: sticky;
  bottom: 0px;
  background-color: var(--color-bg-1);
  border-top: 1px solid var(--color-bg-2);
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
  padding-top: 10px;
  padding-left: 10px;
  padding-right: 10px;
  border-radius: 15px;
}

.info {
  background-color: var(--color-bg-info);
}

.error {
  background-color: var(--color-bg-error);
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

.icon-trash {
  cursor: pointer;
}

.notification:hover .icon-x {
  opacity: 100%;
}
</style>
