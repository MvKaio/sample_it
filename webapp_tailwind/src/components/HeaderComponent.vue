<script setup>
</script>
        
<template>
    <div class="grid grid-cols-4 pt-8">
        <div class="text-2xl px-8 text-center inline">
            <router-link to="/" class="link">Home</router-link>
        </div>
        <div class="text-2xl text-center inline">
            <router-link to="/collections" class="link">All Collections</router-link>
        </div>
        <div class="text-2xl text-center inline">
            <router-link to="/createCollection" class="link">New Collection</router-link>
        </div>
        <div class="text-2xl text-center inline">
            My Samples
        </div>

    </div>
</template>

<style scoped>
a.router-link-active {
    border-bottom: 4px solid rgb(192 132 252);
    border-radius: 4px;
}
</style>