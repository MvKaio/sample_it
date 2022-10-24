<script setup>
</script>
        
<template>
    <div class="grid grid-cols-3 pt-8">
        <div class="text-2xl px-8 text-center inline">
            <router-link to="/" class="link" id="homeLink">Home</router-link>
        </div>
        <div class="text-2xl text-center inline">
            <router-link to="/collections" class="link" id="allCollectionsLink">All Collections</router-link>
        </div>
        <div class="text-2xl text-center inline">
            <router-link to="/createCollection" class="link">New Collection</router-link>
        </div>
    
    </div>
    <div class=" bg-purple-400 rounded-lg h-2 w-4/5 m-auto my-4"></div>
</template>

<style scoped>
a.router-link-active {
    border-bottom: 4px solid rgb(192 132 252);
    border-radius: 4px;
}
</style>