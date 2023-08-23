<script setup>
import { onMounted, shallowRef } from 'vue';
import { invoke } from '@tauri-apps/api';
import { appWindow } from "@tauri-apps/api/window";
import Main from './pages/Main.vue';
import Download from './pages/Download.vue';
import Settings from './pages/Settings.vue';
import About from './pages/About.vue';
import HeaderBar from './components/HeaderBar.vue';

const activePage = shallowRef(Main);

appWindow.listen("page-change", (event) => {
    switch (event.payload) {
        case 'Download':
            activePage.value = Download
            break;
        case 'Settings':
            activePage.value = Settings
            break;
        case 'About':
            activePage.value = About
            break;
        default:
            activePage.value = Main;
    }
});

onMounted(async () => {
    await invoke("show_window");
});
</script>

<template>
    <div class="container" style="height: 100vh; padding: 0px;">
        <HeaderBar />
        <keep-alive>
            <component :is="activePage" />
        </keep-alive>
    </div>
</template>

<style>
@import url("https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap");

:root {
    --color-bg-1: #202020;
    --color-bg-2: #292929;
    --color-bg-3: #2c2c2c;
    --color-shadow: #20202077;
    --color-accent: #5a2a0a;
    --color-hover: #272727;
    --color-text: #ececec;

    font-family: "Noto Sans", sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: var(--color-text);
    background-color: var(--color-bg-3);

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
}

::-webkit-scrollbar {
    width: 12px;
    background-color: var(--color-bg-2);
    border-radius: 5px;
}
::-webkit-scrollbar-thumb {
    background-color: var(--color-bg-3);
    border-radius: 5px;
}
::-webkit-scrollbar-thumb:hover {
    background-color: var(--color-hover);
}

body {
    margin: 0px;
}

.container {
    box-sizing: border-box;
    height: calc(100vh - 80px); /* (Container - HeaderBar) */
    margin: 0px;
    padding: 10px;
    display: flex;
    flex-direction: column;
    justify-content: start;
    text-align: center;
}

.row {
    display: flex;
    justify-content: center;
}

.column {
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.frame {
    padding: 4px;
    border-radius: 4px;
    border: 1px solid transparent;
    background-color: var(--color-bg-1);
    box-shadow: 2px 2px 5px var(--color-shadow);
}

.icon {
    margin-top: auto;
    margin-bottom: auto;
    flex-shrink: 0;
}

input,
button,
.button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-family: inherit;
    background-color: var(--color-bg-1);
    transition: all 0.3s ease;
    box-shadow: 0 2px 2px var(--color-shadow);
}

input:focus {
    outline: none !important;
    border: 1px solid var(--color-accent);
}

button:hover {
    border-color: var(--color-accent);
}

button {
    cursor: pointer;
}
</style>