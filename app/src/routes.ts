import { createMemoryHistory, createRouter } from "vue-router";

import Home from "@views/app/HomePage.vue";
import Favourites from "@views/app/FavouritesPage.vue";
import History from "@views/app/HistoryPage.vue";
import Scheduled from "@views/app/ScheduledPage.vue";
import Library from "@views/app/LibraryPage.vue";
import Voices from "@views/app/VoicesPage.vue";
import Notification from "@views/app/NotificationPage.vue";
import AppLayout from "@views/app/layout.vue";
import AudioPlayer from "@views/app/AudioPlayer.vue";


import AuthenticationLayout from "./views/authentication/layout.vue";
import WelcomePage from "./views/authentication/WelcomePage.vue";
import SignupPage from "./views/authentication/SignupPage.vue";
import LoginPage from "./views/authentication/LoginPage.vue";
import ForgottenPassword from "./views/authentication/ForgottenPassword.vue";
import ConfirmOtpPage from "./views/authentication/ConfirmOtpPage.vue";
import SetNewPassword from "./views/authentication/SetNewPassword.vue";

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
      { path: "player", component: AudioPlayer },
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
      {
        path: "signup",
        component: SignupPage,
      },
      {
        path: "login",
        component: LoginPage,
      },
      {
        path: "forgotten-password",
        component: ForgottenPassword,
      },
      {
        path: "confirm-otp",
        component: ConfirmOtpPage,
      },

      {
        path: "set-new-password",
        component: SetNewPassword,
      },
    ],
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
