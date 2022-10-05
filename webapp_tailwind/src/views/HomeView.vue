<script setup>
import Title from '../components/TitleComponent.vue'
import Header from '../components/HeaderComponent.vue'
</script>
    
<template>
    <div class="flex flex-col text-white max-h-screen h-screen select-none">
        <Title title="Sample-it"></Title>

        <div class="wrapper mx-auto">

            <Header></Header>
            <div class="w-4/5 h-4/5 m-auto pt-4">
                <div class=" bg-purple-400 rounded-lg h-2"></div>

                <p class="text-center text-3xl pt-10">To start, <router-link to="/createCollection"><span class="text-purple-400 hover:text-pink-400 duration-500">create</span></router-link> or <router-link to="/collections"><span class="text-purple-400 hover:text-pink-400 duration-500">import</span></router-link> a new collection, or choose one from the list</p>

                <p class="text-center text-neutral-55 mx-auto py-10 text-2xl font-sans">
                    <router-link to="/collections"><span class=" duration-500 text-purple-400 hover:text-pink-400">Show
                            All
                            Collections</span></router-link>
                </p>

            </div>
        </div>

    </div>
</template>
    
<style scoped>
p {
    text-shadow: black 2px 2px;
    user-select: none;
}
</style>