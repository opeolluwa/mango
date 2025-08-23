<template>
  <template v-if="emptyLibrary">
    <div
      class="overflow-hidden bg-cover bg-no-repeat relative flex flex-col justify-center items-center px-4 h-[90%]"
    >
      <h2 class="text-xl font-medium text-center dark:text-gray-400">
        No audio book has been created
      </h2>
      <p class="text-center mt-1 mb-3 text-gray-500">
        Your audio books will appear as soon as you begin to add them
      </p>

      <UModal
        title="Select a document"
        description="Pick a PDF document not less than 5 MiB"
      >
        <button
          class="bg-app-orange text-app-dark btn-lg inline-flex gap-x-2 items-center px-8 py-3 mt-2 cursor-pointer shadow-md transition-colors duration-200 ease-linear hover:opacity-95 hover:scale-95 control rounded"
        >
          <UIcon name="cuida:plus-circle-outline" class="size-5" />
          Create
        </button>

        <template #body>
          <UFileUpload
            v-model="file"
            accept="application/pdf"
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
    </div>
  </template>
  <template v-else>
    <ScreenLayout>
      <div class="flex flex-col overflow-x-hidden pb-48">
        <div class="flex justify-between">
          <div>
            <h2
              class="text-2xl= font-black text-app-dark/90 dark:text-gray-200"
            >
              Hey, {{ firstName || "there" }}! 👋
            </h2>
            <VueGreetings class="leading-5 text-gray-600 dark:text-gray-400" />
          </div>
        </div>
      </div>

      <button
        class="size-12 bg-app-orange-500 z-[5000] absolute right-8 bottom-24 rounded-full shadow-xl"
      >
        <Icon
          icon="cuida:-plus-circle-outline"
          class="size-4 text-app-dark mx-auto"
        />
      </button>
    </ScreenLayout>
  </template>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";
import VueGreetings from "../../components/uiBlocks/VueGreetings.vue";
import axios from "axios";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";
import { useTokenStore } from "../../stores/token.ts";
import { useUserInformationStore } from "../../stores/user.ts";
import ScreenLayout from "./ScreenLayout.vue";
import { Icon } from "@iconify/vue";

const userInformationStore = useUserInformationStore();
const firstName = computed(() => userInformationStore.firstName);

const emptyLibrary = false;
const file = ref(null);

const uploading = ref(false);
const uploadError = ref<string | null>(null);

const handleUpload = async () => {
  if (!file.value) {
    uploadError.value = "Please select a file first.";
    return;
  }

  const formData = new FormData();
  formData.append("document", file.value);
  const tokenStore = useTokenStore();
  const token = tokenStore.accessToken;
  try {
    uploading.value = true;
    uploadError.value = null;

    const response = await axios.post("/books", formData, {
      headers: {
        "Content-Type": "multipart/form-data",
        Authorization: `Bearer ${token}`,
      },
    });

    console.log("Upload success:", response.data);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    uploadError.value = error.response?.data?.message || "Upload failed";
  } finally {
    uploading.value = false;
  }
};
</script>

<style scoped></style>
