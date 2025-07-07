import { createMemoryHistory, createRouter } from "vue-router";

import Home from "@views/app/HomePage.vue";
import Favourites from "@views/app/FavouritesPage.vue";
import History from "@views/app/HistoryPage.vue";
import Scheduled from "@views/app/ScheduledPage.vue";
import Library from "@views/app/LibraryPage.vue";
import Voices from "@views/app/VoicesPage.vue";
import Notification from "@views/app/NotificationPage.vue";
import AppLayout from "@views/app/layout.vue";

import AuthenticationLayout from "./views/authentication/layout.vue";
import WelcomePage from "./views/authentication/WelcomePage.vue";

const routes = [
  {
    path: "/app",
    children: [
      { path: "", component: Home },
      { path: "favourites", component: Favourites },
      { path: "library", component: Library },
      { path: "history", component: History },
      { path: "voices", component: Voices },
      { path: "scheduled", component: Scheduled },
      { path: "notification", component: Notification },
    ],
    component: AppLayout,
  },
  {
    path: "/",
    coponent: AuthenticationLayout,
    children: [
      {
        path: "",
        component: WelcomePage,
      },
    ],
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
