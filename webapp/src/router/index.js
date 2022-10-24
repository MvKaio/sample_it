import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
  {
    path: "/",
    name: "Home",
    component: () => import("../views/HomeView.vue"),
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
    path: "/collection/:id/:edit?",
    name: "Collection",
    component: () => import("../views/CollectionView.vue"),
  },
  {
    path: "/generateSample/:id",
    name: "Generate Sample",
    component: () => import("../views/GenerateSampleView.vue"),
  },
  {
    path: "/item/:collectionid/:itemid/:edit?",
    name: "View Item",
    component: () => import("../views/ItemView.vue"),
  },
  {
    path: "/addItem/:id/",
    name: "Add Item",
    component: () => import("../views/AddItemView.vue"),
  },
];
const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
