<script setup>
import { ref, reactive } from "vue";
import { IconDotsVertical, IconFolder, IconRefresh } from "@tabler/icons-vue";

import { filterColumnsDirView } from "../helpers";

const files = reactive([{ path: "blabla", size: "1111" }, { path: "blabla", size: "1111" }]);
const showFilterMenu = ref(false);
const dirPanelColumns = reactive([]);
</script>

<template>
  <div class="container">
    <div class="directory-panel">
      <div class="row" style="gap: 10px;">
        <button style="padding: 2px 6px;">
          <IconFolder size="20" color="var(--color-text)" class="icon" style="margin-top: 4px;"/>
        </button>
        <button style="padding: 2px 6px;">
          <IconRefresh size="20" color="var(--color-text)" class="icon" style="margin-top: 4px;"/>
        </button>
        <button style="padding: 2px 6px; margin-left: auto;">
          <span style="color: var(--color-text);">Fetch Tags</span>
        </button>
      </div>
      <div class="frame">
        <table>
          <thead class="table-header">
            <tr>
              <th style="width: 20px; position: relative;">
                <IconDotsVertical size="18" class="icon table-filter-btn" :class="{ 'filter-btn-expanded': showFilterMenu }" @click="showFilterMenu = !showFilterMenu"/>
                <div class="filter-menu" v-if="showFilterMenu" @click.stop>
                  <div class="filter-menu-arrow"></div>
                  <div v-for="column in filterColumnsDirView" :key="column.key">
                    <label>
                      <input class="filterItemInput" type="checkbox" @change="saveFilterColumn(column)" :disabled="column.readonly" v-model="column.enabled"/>
                      {{ column.label }}
                    </label>
                  </div>
                </div>
              </th>
              <th><!-- Reserved for image --></th>
              <th v-for="column in filterColumnsDirView" :key="column.key" v-show="column.enabled">{{ column.label }}</th>
            </tr>
          </thead>
          <tbody>
            <template v-for="file in files" :key="file.path">
              <tr>
                <td><!-- Empty cell reserved for table filter --></td>
                <td class="img-container">
                  <img src="">
                </td>
                <td v-show="filterColumnsDirView.find((col) => col.key === 'filename' && col.enabled)">{{ file.path }}</td>
                <td v-show="filterColumnsDirView.find((col) => col.key === 'size' && col.enabled)">{{ file.size }}</td>
              </tr>
            </template>
          </tbody>
        </table>
      </div>
    </div>
    <div class="tags-panel">
      <div class="row">
        
      </div>
    </div>
  </div>
</template>

<style scoped>
.directory-panel {
  flex-grow: 1;
}

table {
  width: 100%;
  border-collapse: separate;
  border-spacing: 0px 4px;
}

tbody {
  font-size: 0.95em;
}

tbody tr td:nth-child(1) {
  border-top-left-radius: 8px;
  border-bottom-left-radius: 8px;
}

tbody tr td:last-child {
  border-top-right-radius: 8px;
  border-bottom-right-radius: 8px;
}

tbody tr:hover {
  background-color: var(--color-hover);
}

.table-header {
  font-style: italic;
  background-color: var(--color-bg-1);
  position: sticky;
  top: 0;
  z-index: 1;
}

.table-header th {
  font-weight: 500;
  border-bottom: 1px solid var(--color-bg-2);
  text-wrap: nowrap;
}

tbody td:not(:last-child) {
  border-right: 1px solid var(--color-hover);
}

.table-filter-btn {
  margin-top: 4px;
  padding: 2px;
  cursor: pointer;
  border-radius: 10px;
  border: 1px solid transparent;
  transform: rotate(0deg);
  transition: all 0.2s ease;
}

.table-filter-btn:hover {
  border: 1px solid var(--color-accent);
  background-color: var(--color-hover);
}

.expanded {
  display: flex;
  width: 200px;
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
