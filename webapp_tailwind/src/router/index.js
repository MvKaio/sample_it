import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";

const routes = [];

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "Home",
      component: HomeView,
    },
    {
      path: "/collections",
      name: "Collections",
      component: () => import("../views/AllCollectionsView.vue"),
    },
    {
      path: "/createCollection",
      name: "Create Collection",
      component: () => import("../views/CreateCollectionView.vue"),
    },
  ],
});

export default router;
