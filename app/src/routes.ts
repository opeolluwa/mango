import { createMemoryHistory, createRouter } from "vue-router";

import Home from "./views/Home.vue";
import Favourites from "./views/Favourites.vue";
import History from "./views/History.vue";
import Scheduled from "./views/Scheduled.vue";
const routes = [
  {
    path: "/",
    children: [
      { path: "", component: Home },
      { path: "/favouries", component: Favourites },
      { path: "/history", component: History },
      { path: "/scheduled", component: Scheduled },
    ],
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
