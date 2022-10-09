<script setup>
</script>
    
<template>
    <div class="component-container">

        <p class="text-center text-3xl pt-10">To start, <router-link to="/createCollection"><span
                    class="text-purple-400 hover:text-pink-400 duration-500">create</span></router-link> or <router-link
                to="/collections"><span class="text-purple-400 hover:text-pink-400 duration-500">import</span>
            </router-link> a new collection, or choose one from the list</p>

        <p class="text-center text-neutral-55 mx-auto py-10 text-2xl font-sans">
            <router-link to="/collections"><span class=" duration-500 text-purple-400 hover:text-pink-400">Show
                    All
                    Collections</span></router-link>
        </p>

    </div>
</template>
    
<style scoped>
p {
    text-shadow: black 2px 2px;
    user-select: none;
}
</style>