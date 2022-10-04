<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
</script>

<template>
  <div class="max-h-screen h-screen bg">
    <router-view v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
</template>

<style>
/* .logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
} */

h1 {
  text-shadow: black 5px 4px;
  user-select: none;
}

/* .bg {
  width: 100%;
  height: 100vh;
  background-size: 300% 300%;
  background-image: linear-gradient(-45deg,
      rgb(187, 187, 187) 0%,
      rgb(81, 81, 81) 25%,
      rgb(95, 95, 95) 51%,
      rgb(58, 58, 58) 100%);
  animation: AnimateBG 10s ease infinite;
} */

.bg {
  width: 100%;
  height: 100vh;
  background-size: 300% 300%;
  background-image: linear-gradient(-45deg,
      rgb(192 132 255) 0%,
      rgb(79, 51, 109) 25%,
      rgb(83, 53, 115) 51%,
      rgb(50, 33, 72) 100%);
  animation: AnimateBG 8s ease infinite;
}

@keyframes AnimateBG {
  0% {
    background-position: 0% 50%;
  }

  50% {
    background-position: 100% 50%;
  }

  100% {
    background-position: 0% 50%;
  }


}

.fade-enter-active,
.fade-leave-active {
  transition: all 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
