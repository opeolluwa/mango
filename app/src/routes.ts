import { createRouter, createWebHistory } from "vue-router";

import AudioPlayer from "@views/player/AudioPlayer.vue";
import Favourites from "@views/app/FavouritesPage.vue";
import History from "@views/app/HistoryPage.vue";
import Home from "@views/app/HomePage.vue";
import AppLayout from "@views/app/AppLayout.vue";
import Library from "@views/app/LibraryPage.vue";
import Notification from "@views/app/NotificationPage.vue";
import Voices from "@views/app/VoicesPage.vue";

import ConfirmOtpPage from "./views/authentication/ConfirmOtpPage.vue";
import ForgottenPassword from "./views/authentication/ForgottenPassword.vue";
import AuthenticationLayout from "./views/authentication/AutenticationLayout.vue";
import LoginPage from "./views/authentication/LoginPage.vue";
import LoginExistingPage from "./views/authentication/LoginExistingPage.vue";
import SetNewPassword from "./views/authentication/SetNewPassword.vue";
import SignupPage from "./views/authentication/SignupPage.vue";
import OnboardingPage from "./views/authentication/OnboardingPage.vue";
import VerifyAccountRecovery from "./views/authentication/VerifyAccountRecovery.vue";

import ScreenOne from "./views/walkthrough/ScreenOne.vue";
import ScreenTwo from "./views/walkthrough/ScreenTwo.vue";
import ScreenThree from "./views/walkthrough/ScreenThree.vue";

import AudioPlayerLayout from "./views/player/AudioPlayerLayout.vue";
import UserProfile from "./views/app/settings/UserProfile.vue";
import AppSettingsLayout from "./views/app/settings/AppSettingsLayout.vue";
import SettingsPage from "./views/app/settings/SettingsPage.vue";
import SecurityPage from "./views/app/settings/security/SecurityPage.vue";
import HelpPage from "./views/app/settings/HelpPage.vue";
import PaymentPage from "./views/app/settings/PaymentPage.vue";
import AppSecurityLayout from "./views/app/settings/security/AppSecurityLayout.vue";
import UpdatePassword from "./views/app/settings/security/UpdatePassword.vue";

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
      {
        path: "favourites",
        component: Favourites,
        name: "Favourites",
        meta: { label: "Favourites" },
      },
      {
        path: "library",
        component: Library,
        name: "Library",
        meta: { label: "Library" },
      },
      {
        path: "recent",
        component: History,
        name: "Recent",
        meta: { label: "Recent Books" },
      },
      {
        path: "voices",
        component: Voices,
        name: "Voices",
        meta: { label: "Actors" },
      },
      {
        path: "notification",
        component: Notification,
        name: "Notification",
        meta: { label: "Notification" },
      },
      {
        path: "app-settings",
        component: AppSettingsLayout,
        name: "AppSettings",
        children: [
          {
            path: "",
            component: SettingsPage,
            name: "Settings",
            meta: { label: "Settings" },
          },
          {
            path: "user-profile",
            component: UserProfile,
            name: "UserProfile",
            meta: { label: "Profile" },
          },
          {
            path: "security-and-privacy",
            component: AppSecurityLayout,
            name: "AppSecurity",
            meta: { label: "Security & Privacy" },
            children: [
              { path: "", component: SecurityPage, name: "SecurityAndPrivacy" },
              {
                path: "update-accout-password",
                component: UpdatePassword,
                name: "UpdatePassword",
                meta: { label: "Update password" },
              },
            ],
          },
          {
            path: "help-and-support",
            component: HelpPage,
            name: "HelpAndSupport",
            meta: { label: "Help & Support" },
          },
          {
            path: "payment-and-subscription",
            component: PaymentPage,
            name: "PaymentAndSubscription",
            meta: { label: "Payment & Subscription" },
          },
        ],
      },
    ],
  },
  {
    path: "/player",
    name: "AppPlayer",
    component: AudioPlayerLayout,
    children: [{ path: "", component: AudioPlayer, name: "Player" }],
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
        path: "login-existing",
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
        path: "verify-account",
        component: VerifyAccountRecovery,
        name: "VerifyAccountRecovery",
      },
      {
        path: "set-new-password",
        component: SetNewPassword,
        name: "SetNewPassword",
      },
      {
        path: "onboarding",
        name: "Onboarding",
        component: OnboardingPage,
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
