<script setup>
import { ref } from "vue";
import { IconCaretRightFilled } from "@tabler/icons-vue";

const header = ref(null);
const body = ref(null);

function toggleExpandGroup() {
    body.value.classList.toggle("expanded");
    header.value.children[0].classList.toggle("icon-expanded");
}

</script>

<template>
    <div class="column setting-column">
        <div class="row setting-btn" ref="header" @click="toggleExpandGroup">
            <IconCaretRightFilled size="30" class="icon"/>
            <slot name="head">

            </slot>
        </div>
        <div class="row setting-body" ref="body">
            <slot name="body">

            </slot>
        </div>
    </div>
</template>

<style scoped>
.setting-column {
  padding-top: 20px;
  padding-bottom: 20px;
  margin-left: 5%;
  margin-right: 5%;
  border-bottom: 1px solid #4c4c4c;
}

.setting-btn {
  justify-content: start;
  cursor: pointer;
}

.setting-body {
  display: none;
  height: 0px;
  padding-top: 20px;
}

.expanded {
  display: inherit;
  height: inherit;
}

.icon {
    transform: rotate(0deg);
    transition: transform 0.2s ease-in-out;
}

.icon-expanded {
    transform: rotate(90deg);
}
</style>
