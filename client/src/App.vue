<script setup lang="ts">
import axios from 'axios';
import { onMounted, ref } from 'vue';

let baseUrl = 'http://192.168.1.7:3000';
let videos = ref([]);
function fetchVideos() {
  axios.get(baseUrl + '/videos')
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
    <div class="row">
      <div class="col-md-12">
        <ul class="list-group">
          <li v-for="video in videos" :key="video" class="list-group-item">
            <div class="d-flex w-100 justify-content-between">
              <a :href="baseUrl + '/play/' + video" target="_blank">{{ video }}</a>
            </div>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  margin-top: 50px;
}

.row {
  margin-bottom: 20px;
}

.list-group-item {
  cursor: pointer;
}

.list-group-item:hover {
  background-color: #f5f5f5;
}

.d-flex {
  justify-content: space-between;
}

.w-100 {
  width: 100%;
}

.justify-content-between {
  justify-content: space-between;
}

.mb-1 {
  margin-bottom: 0.25rem;
}
</style>
