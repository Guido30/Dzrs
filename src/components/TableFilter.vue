<script setup>
import { ref, toRef, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { IconDotsVertical } from "@tabler/icons-vue";

const props = defineProps(["columns"]);
const columns = toRef(props.columns);

// Visibility toggle for the filter options
const expanded = ref(false);

async function saveFilterCol(col) {
  await invoke("config_set", { key: col.config, value: `${col.enabled}` }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "saveFilterCol", msg: err }));
}

onMounted(() => {
  // Listener for closing the filter menu when clicking outside of it
  document.addEventListener("click", (event) => {
    if (!event.target.closest(".filter-btn")) {
      expanded.value = false;
    }
  });
});
</script>

<template>
  <IconDotsVertical size="18" class="icon filter-btn" :class="{ 'filter-btn-expanded': expanded }" @click="expanded = !expanded" />
  <div class="filter-menu" v-if="expanded" @click.stop>
    <div class="filter-menu-arrow"></div>
    <div v-for="col in columns" :key="col.key">
      <label>
        <input type="checkbox" @change="saveFilterCol(col)" :disabled="col.readonly" v-model="col.enabled" />
        {{ col.label }}
      </label>
    </div>
  </div>
</template>

<style scoped>
.filter-btn {
  margin-top: 4px;
  padding: 2px;
  cursor: pointer;
  border-radius: 10px;
  border: 1px solid transparent;
  transform: rotate(0deg);
  transition: all 0.2s ease;
}

.filter-btn:hover {
  border: 1px solid var(--color-accent);
  background-color: var(--color-hover);
}

.filter-btn-expanded {
  transform: rotate(90deg);
}

.filter-menu {
  position: absolute;
  min-width: max-content;
  text-align: left;
  margin-top: 8px;
  padding: 10px;
  padding-top: 5px;
  padding-bottom: 5px;
  background-color: var(--color-bg-2);
  border: 1px solid var(--color-accent);
  border-radius: 10px;
  border-top-left-radius: 4px;
  z-index: 2;
}

.filter-menu-arrow {
  width: 0;
  height: 0;
  border-top: 0px solid transparent;
  border-bottom: 15px solid var(--color-accent);
  border-left: 10px solid transparent;
  border-right: 10px solid transparent;
  position: absolute;
  top: -15px;
  left: 1px;
}

.filter-menu label {
  color: var(--color-text);
  font-size: 1em;
  font-weight: 400;
  user-select: none;
}
</style>
