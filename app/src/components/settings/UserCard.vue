<template>
  <UModal
    v-model:open="open"
    title="Select a document"
    description="Pick a PDF document not less than 5 MiB"
  >
    <template #body>
      <UFileUpload
        v-model="file"
        accept="image/*"
        :multiple="false"
        class="min-h-48"
      />

      <div class="mt-4">
        <button
          :disabled="uploading"
          class="bg-app-orange text-app-dark btn-lg inline-flex gap-x-2 items-center px-8 py-3 mt-2 cursor-pointer shadow-md transition-colors duration-200 ease-linear hover:opacity-95 hover:scale-95 control rounded w-full text-center justify-center"
          @click="handleUpload"
        >
          <span v-if="!uploading">Upload</span>
          <span v-else>Uploading...</span>
        </button>

        <ErrorOutlet v-if="uploadError" class="mt-2">
          {{ uploadError }}
        </ErrorOutlet>
      </div>
    </template>
  </UModal>

  <div class="flex items-center gap-x-4 relative">
    <AvatarRoot
      :class="[
        'bg-blackA3 inline-flex size-16 select-none items-center justify-center overflow-hidden rounded-full align-middle',
      ]"
      :style="{ width: `${avatarSize}px`, height: `${avatarSize}px` }"
    >
      <AvatarImage
        class="h-full w-full rounded-[inherit] object-cover"
        :src="userInformation.profilePicture??''"
        alt="Colm Tuite"
      />
      <AvatarFallback
        class="text-grass11 dark:text-stone-300 leading-1 flex h-full w-full items-center justify-center bg-app-orange-50 dark:bg-stone-800 text-sm font-medium"
      >
        {{ initials }}
      </AvatarFallback>

      <span
        v-if="editable"
        class="text-xs absolute -bottom-1.5 -right-1.5 bg-white/80 p-1 rounded-full text-app-orange-500"
      >
        <Icon icon="tabler:edit" class="size-4 text-app" @click="uploadImage" />
      </span>
    </AvatarRoot>

    <div v-if="showText" class="flex flex-col">
      <div class="text-2xl font-bold dark:text-white/70">
        {{ fullName }}
      </div>
      <p class="text-gray-400 font-medium">{{ userInformation.email }}</p>
      <span class="text-sm font-semibold text-gray-600">Basic plan</span>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { AvatarFallback, AvatarImage, AvatarRoot } from "reka-ui";
import { computed, ref, toRefs } from "vue";
import { useUserInformationStore } from "../../stores/user";
import ErrorOutlet from "../form/ErrorOutlet.vue";
import { useTokenStore } from "../../stores/token";
import axios from "axios";

const props = defineProps({
  showText: { type: Boolean, default: true },
  avatarSize: { type: Number, default: 75 },
  editable: { type: Boolean, default: false },
});

const open = ref(false);
const openModal = () => {
  open.value = true;
};

const file = ref(null);

const userInformationStore = useUserInformationStore();

const uploading = ref(false);
const uploadError = ref<string | null>(null);

const handleUpload = async () => {
  if (!file.value) {
    uploadError.value = "Please select a file first.";
    return;
  }

  const formData = new FormData();
  formData.append("image", file.value);
  const tokenStore = useTokenStore();

  const token = tokenStore.accessToken;
  try {
    uploading.value = true;
    uploadError.value = null;

    const response = await axios.post("/user/avatar", formData, {
      headers: {
        "Content-Type": "multipart/form-data",
        Authorization: `Bearer ${token}`,
      },
    });


    userInformationStore.setProfilePicture(response.data.data.profilePicture);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    uploadError.value = error.response?.data?.message || "Upload failed";
  } finally {
    uploading.value = false;
    open.value = false;
    file.value = null;
  }
};

const { showText, editable, avatarSize } = toRefs(props);

const userInformation = computed(() => userInformationStore.user);
const fullName = computed(() => userInformationStore.fullName);
const initials = computed(() =>
  (userInformationStore.user.firstName?.[0] ?? "").concat(
    userInformationStore.user.lastName?.[0] ?? ""
  )
);

const uploadImage = () => {
  if (editable.value) {
    openModal();
  }
};
</script>
