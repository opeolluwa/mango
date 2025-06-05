import { createMemoryHistory, createRouter } from "vue-router";

import Home from "./views/Home.vue";
import Favourites from "./views/Favourites.vue";
import History from "./views/History.vue";
import Scheduled from "./views/Scheduled.vue";
import Library from "./views/Library.vue";
import Voices from "./views/Voices.vue";
import Notification from "./views/Notification.vue";
const routes = [
  {
    path: "/",
    children: [
      { path: "", component: Home },
      { path: "/favouries", component: Favourites },
      { path: "/library", component: Library },
      { path: "/history", component: History },
      { path: "/voices", component: Voices },
      { path: "/scheduled", component: Scheduled },
      { path: "/notification", component: Notification },
    ],
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
