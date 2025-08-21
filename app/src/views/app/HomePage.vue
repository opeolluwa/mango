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
    <div class="flex flex-col overflow-x-hidden pb-48">
      <div class="flex justify-between">
        <div>
          <h2 class="text-2xl font-black text-app-dark/90 dark:text-gray-200">
            Hey, {{ firstName || "there" }}! 👋
          </h2>
          <VueGreetings class="leading-5 text-gray-600 dark:text-gray-400" />
        </div>

        <div class="col-span-4 flex gap-x-3 items-center">
          <AvatarRoot
            class="bg-blackA3 inline-flex size-8 select-none items-center justify-center overflow-hidden rounded-full align-middle"
          >
            <AvatarImage
              class="h-full w-full rounded-[inherit] object-cover"
              src="https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?&w=128&h=128&dpr=2&q=80"
              alt="Colm Tuite"
            />
            <AvatarFallback
              class="text-grass11 dark:text-stone-300 leading-1 flex h-full w-full items-center justify-center bg-white dark:bg-stone-800 text-sm font-medium"
              :delay-ms="600"
            >
              CT
            </AvatarFallback>
          </AvatarRoot>
        </div>
      </div>

      <div v-if="!emptyLibrary" class="mt-12">
        <h3 class="font-3xl text-gray-400 font-medium mb-3">
          Continue listening
        </h3>
        <div
          class="mb-10 flex gap-x-4 w-screen overflow-x-scroll pr-24"
          style="scroll-behavior: smooth"
        ></div>
      </div>

      <section v-if="!emptyLibrary" class="mt-8">
        <h3 class="font-3xl text-gray-400 font-medium mb-2">Recently added</h3>

        <div class="hiden">
          <!-- <AudioBook
            v-for="book in audiobooks"
            :key="book.id"
            :title="book.title"
            :author="book.author"
            :cover="book.cover"
            class="hover:shadow-md transition-shadow cursor-pointer"
          /> -->
        </div>
      </section>
    </div>
  </template>
</template>

<script lang="ts" setup>
import { AvatarFallback, AvatarImage, AvatarRoot } from "reka-ui";
import { computed, ref } from "vue";
import VueGreetings from "../../components/uiBlocks/VueGreetings.vue";
import { useCachedUserStore } from "../../stores/cachedUser.ts";
import axios from "axios";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";
import { useTokenStore } from "../../stores/token.ts";
const store = useCachedUserStore();
const firstName = computed(() => store.firstName);

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
  // formData.append("t")

  const tokenStore = useTokenStore();
  const token = await tokenStore.extractAccessToken();
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
