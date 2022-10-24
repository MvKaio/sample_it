<script setup>
import Header from './components/HeaderComponent.vue'
</script>

<template>
  <div class="max-h-screen h-screen bg text-white flex items-center overflow-auto">
    <div class="wrapper mx-auto">
      <Header></Header>
      <Suspense>
        <template #default>
          <router-view v-slot="{ Component }">
            <transition name="fade" mode="out-in">
              <component :is="Component" />
            </transition>
          </router-view>
        </template>
        <template #fallback>
          <div class="component-container flex justify-center items-center">
            <div class="lds-ellipsis">
              <div></div>
              <div></div>
              <div></div>
              <div></div>
            </div>
          </div>
        </template>
      </Suspense>
    </div>
  </div>
</template>

<style>
.wrapper {
  width: 95%;
  height: 95%;
  border-radius: 20px;
  box-shadow: 2rem 2rem 3rem 2rem black;
  overflow: auto;
  /* background-color: var(--background-color); */
}

.bg {
  width: 100%;
  height: 100vh;
  background-size: 150% 150%;
  background-image: linear-gradient(-45deg,
      rgb(187, 187, 187) 0%,
      rgb(136, 136, 136) 25%,
      rgb(95, 95, 95) 51%,
      rgb(58, 58, 58) 100%);
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
