<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { useRoute } from 'vue-router';
import { rootStore } from "../states/root";
import Hammer from "hammerjs";

const route = useRoute();
const root = rootStore();
const filename = ref();
const videoPath = ref()
const video = ref();

let hammertime: HammerManager;

onMounted(() => {
    filename.value = route.params.filename;
    videoPath.value = root.baseUrl + '/video/' + filename.value;

    initHammer();
});

onUnmounted(() => {
    hammertime.destroy();
});

function initHammer() {
    hammertime = new Hammer(video.value)
    hammertime.get('swipe').set({ direction: Hammer.DIRECTION_ALL })

    hammertime.on('swipeleft', (e: HammerInput) => {
        // 回退10秒
        video.value.currentTime += e.deltaX
    })

    hammertime.on('swiperight', (e: HammerInput) => {
        // 向前跳过10秒
        video.value.currentTime += e.deltaX
    })

    // 手势调节音量示例（需自行根据实际需求调整）
    hammertime.on('swiperup', (e: HammerInput) => {
        video.value.volume += e.deltaY * 0.01;
    })

    hammertime.on('swiperdown', (e: HammerInput) => {
        video.value.volume += e.deltaY * 0.01;
    })
}
</script>

<template>
    <div class="container">
        <div>
            <h1>{{ filename }}</h1>
        </div>
        <div>
            <video ref="video" :src="videoPath" controls autoplay="false" volume="1" muted="true" width="80%" />
        </div>
    </div>
</template>

<style lang="scss" scoped>
.container {
    align-items: center;
}
</style>