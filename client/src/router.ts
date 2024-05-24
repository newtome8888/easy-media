import { createRouter, createWebHistory } from "vue-router";
import Video from "./pages/Video.vue";
import Home from "./pages/Home.vue";

// Routes
const routes = [
    {
        path: "/", 
        name: "home", 
        component: Home
    },
    {
        path: "/video/:filename", 
        name: "video", 
        component: Video
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;