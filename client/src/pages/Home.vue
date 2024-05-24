<script lang="ts" setup>
import axios from 'axios';
import { onMounted, ref } from 'vue';
import { rootStore } from '../states/root';

const root = rootStore();
let videos = ref([]);
function fetchVideos() {
    axios.get(root.baseUrl + '/videos')
        .then(res => {
            console.log(res.data);
            videos.value = res.data;
        })
        .catch(err => {
            console.log(err);
        })
}

onMounted(() => {
    fetchVideos();
})
</script>

<template>
    <div class="container">
        <ul>
            <nav>
                <li v-for="video in videos" :key="video">
                    <router-link :to="{ name: 'video', params: { filename: video } }">{{ video }}</router-link>
                </li>
            </nav>
        </ul>
    </div>
</template>
<style scoped lang="scss">
.container{
    width: 80%;
    height: 80%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}
</style>