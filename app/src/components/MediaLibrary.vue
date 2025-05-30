<template>
  <div
    class="modal absolute bg-black/60 flex flex-col text-accent-secondary w-screen h-screen z-50 px-8"
    v-show="displayLibrary"
  >
    <form action="" class="text-accent-secondary">
      <div>
        <label for="price" class="block text-sm/6 font-medium text-gray-900"
          >Audio books</label
        >
        <input
          type="search"
          name="price"
          id="price"
          class="block min-w-0 grow pr-3 pl-1 text-base text-gray-900 focus:outline-none sm:text-sm/6 border bg-gray-500 w-full rounded-lg py-2 px-6 placeholder:px-4 placeholder:text-accent-secondary border-accent-secondary cursor-pointer"
          placeholder="search collection"
        />

        <ul class="mt-2">
          <li
            v-for="(audio, index) in audioBooks"
            :key="index"
            class="text-sm font-medium overflow-hidden text-ellipsis cursor-pointer"
          >
            {{ audio.fileName }}
          </li>
        </ul>
      </div>
    </form>
  </div>
</template>
