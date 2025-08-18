import { useUserInformation } from "./stores/user";
import { Purchases } from "@revenuecat/purchases-js";

const userInformationStore = useUserInformation();

const appUserId = userInformationStore.identifier;

Purchases.configure({
  apiKey: "WEB_BILLING_PUBLIC_API_KEY",
  appUserId: appUserId, // optional, can be omitted if you want RevenueCat to generate one
});
