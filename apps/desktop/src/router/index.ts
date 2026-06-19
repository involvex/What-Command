import { createRouter, createWebHistory } from "vue-router";
import BrowseView from "../views/BrowseView.vue";
import PlaygroundView from "../views/PlaygroundView.vue";
import ResearchView from "../views/ResearchView.vue";
import AiChatView from "../views/AiChatView.vue";
import MoreView from "../views/MoreView.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", redirect: "/browse" },
    { path: "/browse", component: BrowseView },
    { path: "/playground", component: PlaygroundView },
    { path: "/research", component: ResearchView },
    { path: "/ai", component: AiChatView },
    { path: "/more", component: MoreView },
  ],
});
