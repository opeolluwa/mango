<template>
    <div class="loader bg-transparent"></div>
</template>

<style scoped>
/* HTML: <div class="loader"></div> */
.loader {
  width: 45px;
  aspect-ratio: .75;
  --c: no-repeat linear-gradient(#D2D0A0 0 0);
  background: 
    var(--c) 0%   50%,
    var(--c) 50%  50%,
    var(--c) 100% 50%;
  background-size: 20% 50%;
  animation: l6 1s infinite linear;
}
@keyframes l6 {
  20% {background-position: 0% 0%  ,50% 50% ,100% 50% }
  40% {background-position: 0% 100%,50% 0%  ,100% 50% }
  60% {background-position: 0% 50% ,50% 100%,100% 0%  }
  80% {background-position: 0% 50% ,50% 50% ,100% 100%}
}
</style>