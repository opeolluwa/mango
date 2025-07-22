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

import ScreenOne from "./views/walkthrough/ScreenOne.vue";
import ScreenTwo from "./views/walkthrough/ScreenTwo.vue";
import ScreenThree from "./views/walkthrough/ScreenThree.vue";

const routes = [
  {
    path: "/",
    name: "ScreenOne",
    component: ScreenOne,
  },
  {
    path: "/screen-two",
    name: "ScreenTwo",
    component: ScreenTwo,
  },
  {
    path: "/screen-three",
    name: "ScreenThree",
    component: ScreenThree,
  },
  {
    path: "/app",
    name: "AppLayout",
    component: AppLayout,
    children: [
      { path: "", component: Home, name: "Home" },
      { path: "favourites", component: Favourites, name: "Favourites" },
      { path: "library", component: Library, name: "Library" },
      { path: "recent", component: History, name: "Recent" },
      { path: "voices", component: Voices, name: "Voices" },
      { path: "settings", component: Settings, name: "Settings" },
      { path: "notification", component: Notification, name: "Notification" },
      { path: "player", component: AudioPlayer, name: "Player" },
    ],
  },
  {
    path: "/auth",
    name: "AuthenticationLayout",
    component: AuthenticationLayout,
    children: [
      { path: "", component: SignupPage, name: "SignUp" },
      { path: "signup", component: SignupPage, name: "SignUp" },
      { path: "login", component: LoginPage, name: "Login" },
      {
        path: "login2",
        component: LoginExistingPage,
        name: "ExistingUserLogin",
      },
      {
        path: "forgotten-password",
        component: ForgottenPassword,
        name: "ForgottenPassword",
      },
      { path: "confirm-otp", component: ConfirmOtpPage, name: "ConfirmOtp" },
      {
        path: "set-new-password",
        component: SetNewPassword,
        name: "SetNewPassword",
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
