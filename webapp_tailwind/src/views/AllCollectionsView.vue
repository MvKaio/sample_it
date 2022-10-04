<script setup>
import Title from '../components/TitleComponent.vue'
import { onMounted, ref } from 'vue'

const collections = ref([])
onMounted(async () => {
    fetch('http://localhost:3000/collections')
        .then(r => r.json())
        .then(r => collections.value = r)
})
</script>
                
<template>
    <div class="flex flex-col text-white max-h-screen h-screen select-none">
        <Title title="Your Collections"></Title>
        <!-- <div class="w-2/5 mx-auto first-letter:items-center"> -->
        <div class="wrapper mx-auto">

            <div class="grid grid-cols-2 pt-8">
                <div class="text-2xl px-8 text-right inline">
                    <router-link to="/" class="link">Home</router-link>
                </div>
                <div class="text-2xl text-left inline">
                    <router-link to="/createCollection" class="link">New Collection</router-link>
                </div>
            </div>

            <div class="w-4/5 h-4/5 m-auto pt-4">
                <div class=" bg-purple-400 rounded-lg h-2"></div>
                <div class="grid grid-cols-2 pt-8">
                    <div class="text-xl text-center inline">Name</div>
                    <div class="text-xl text-center inline">Last Updated</div>
                </div>

                <div class="h-4 ">

                </div>
                <div class="grid grid-cols-2" v-for="collection in collections" :key="collection.id">
                    <div class="text-xl text-center inline">{{collection.name}}</div>
                    <div class="text-xl text-center inline">{{collection.lastUpdated}}</div>
                </div>
            </div>
        </div>
    </div>
</template>
    
<style scoped>

</style>
        