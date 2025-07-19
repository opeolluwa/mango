import { createRouter, createWebHistory } from "vue-router";

import AudioPlayer from "@views/app/AudioPlayer.vue";
import Favourites from "@views/app/FavouritesPage.vue";
import History from "@views/app/HistoryPage.vue";
import Home from "@views/app/HomePage.vue";
import AppLayout from "@views/app/layout.vue";
import Library from "@views/app/LibraryPage.vue";
import Notification from "@views/app/NotificationPage.vue";
import Settings from "@views/app/SettingsPage.vue";
import Voices from "@views/app/VoicesPage.vue";

import ConfirmOtpPage from "./views/authentication/ConfirmOtpPage.vue";
import ForgottenPassword from "./views/authentication/ForgottenPassword.vue";
import AuthenticationLayout from "./views/authentication/layout.vue";
import LoginPage from "./views/authentication/LoginPage.vue";
import LoginExistingPage from "./views/authentication/LoginExistingPage.vue";
import SetNewPassword from "./views/authentication/SetNewPassword.vue";
import SignupPage from "./views/authentication/SignupPage.vue";
import WelcomePage from "./views/authentication/WelcomePage.vue";

const routes = [
  {
    path: "/app",
    children: [
      { path: "", component: Home, name: "home" },
      { path: "favourites", component: Favourites },
      { path: "library", component: Library },
      { path: "recent", name: "recent", component: History },
      { path: "voices", component: Voices },
      { path: "settings", name: "settings", component: Settings },
      { path: "notification", name: "notification", component: Notification },
      { path: "player", component: AudioPlayer },
    ],
    component: AppLayout,
  },
  {
    path: "/",
    coponent: AuthenticationLayout,
    children: [
      { path: "", component: WelcomePage },
      { path: "signup", component: SignupPage },
      { path: "login", component: LoginPage, name: "defaultLogin" },
      {
        path: "login2",
        component: LoginExistingPage,
        name: "existingUserLogin",
      },
      { path: "forgotten-password", component: ForgottenPassword },
      { path: "confirm-otp", component: ConfirmOtpPage },
      { path: "set-new-password", component: SetNewPassword },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
