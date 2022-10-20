import { createRouter, createWebHashHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";

const router = createRouter({
  history: createWebHashHistory(),
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
    {
      path: "/addItem",
      name: "Add Item",
      component: () => import("../views/AddItemView.vue"),
    },
    {
      path: "/collection/:id",
      name: "Collection",
      component: () => import("../views/CollectionView.vue"),
    },
    {
      path: "/generateSample/:id",
      name: "Generate Sample",
      component: () => import("../views/GenerateSampleView.vue"),
    },
    {
      path: "/editCollection/:id",
      name: "EditCollection",
      component: () => import("../views/EditCollectionView.vue"),
    }
  ],
  mode: "history",
});

export default router;
