<script setup>
import { ref, onMounted } from "vue";
import { IconBrandGithub } from "@tabler/icons-vue";
import { getVersion } from "@tauri-apps/api/app";

import HeaderBar from "../components/HeaderBar.vue";

const appVersion = ref();

onMounted(async () => {
  appVersion.value = await getVersion().then((ver) => ver);
});
</script>

<template>
  <HeaderBar />
  <div class="container">
    <div class="frame">
      <div class="column">
        <div class="row">
          <img src="../assets/logox256.png" style="margin-right: 20px; user-select: none" />
          <div class="column" style="padding-left: 40px; text-align: left; border-left: 1px solid var(--color-text)">
            <h1>Dzrs</h1>
            <p style="margin-top: 0px; margin-bottom: 0px">Made by Guido30</p>
            <span>v{{ appVersion }}</span>
          </div>
        </div>
        <div class="column" style="align-items: start; padding-left: 40px">
          <a href="https://github.com/Guido30/Dzrs.git" target="_blank">
            <div class="row">
              <IconBrandGithub class="icon" size="40" color="var(--color-text)" />
              <p style="padding-left: 10px; user-select: none">Github</p>
            </div>
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.frame {
  margin: auto;
  padding: 10%;
  border-radius: 50px;
  box-shadow: 5px 8px 10px var(--color-shadow);
}

h1 {
  margin-bottom: 5px;
}

a {
  font-size: 1.4em;
  color: var(--color-text);
  text-decoration: none;
  border-radius: 30px;
  padding-left: 20px;
  padding-right: 20px;
  transition: background-color 0.2s ease;
}

a:hover {
  background-color: var(--color-bg-2);
}
</style>
