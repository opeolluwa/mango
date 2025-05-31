<template>
      <div
        id="wrapper"
        class="flex flex-col items-center gap-x-2 gap-y-2 justify-evenly absolute left-0 right-0 bottom-5 mb-4 hidden"
        v-for="(audio, index) in audioBooks"
        :key="index"
        v-bind:class="{ isActive: isCurrentSong(index) }"
      >

        <div id="info" class="text-center">
          <p class="leading-loose">You are listening to...</p>
          <h2 class="text-xl my-2 font-medium overflow-hidden text-ellipsis">
            {{ audio.fileName }}
          </h2>
        </div>

        <audio :src="audioBooks[index]?.audioSrc"></audio>
        <div id="progress" class="flex flex-col px-12">
          <div class="flex justify-between mb-1 prose-sm">
            <small> 0.00 </small>
            <small>6.25</small>
          </div>
          <Progressbar :progress="100" />
        </div>

        <div id="controls" class="flex items-center justify-evenly">
          <button
            @click="gotoPreviousSong"
            class="size-8 disabled:text-accent/95 control"
            :disabled="index == 0"
          >
            <BackwardIcon />
          </button>

          <button
            class="bg-accent-secondary text-accent flex justify-center items-center rounded-full size-16 active:scale-75 transition-all duration-75 ease-linear p-[5px]"
            @click="playTestfile"
          >
            <!-- <button
            class="bg-accent-secondary text-accent flex justify-center items-center rounded-full size-16 active:scale-75 transition-all duration-75 ease-linear p-[5px]"
            @click="changeSong(index)"
          > -->
            <PlayIcon
              class="size-8 transition duration-150 ease-in-out"
              v-show="!isPlaying"
            />
            <PauseIcon
              class="size-8 transition duration-150 ease-in-out"
              v-show="isPlaying"
            />
          </button>
          <button
            class="size-8 disabled:text-accent/95 control"
            :disabled="index == totalBooks - 1"
            @click="gotoNextSong"
          >
            <ForwardIcon />
          </button>
        </div>
      </div>
</template>