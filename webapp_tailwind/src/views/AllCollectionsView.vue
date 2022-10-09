<script setup>

import { onMounted, ref } from 'vue'
import store from '../store';

const collections = ref([])
onMounted(async () => {
    fetch('http://localhost:3000/collections')
        .then(r => r.json())
        .then(r => collections.value = r)
})

console.log(store.state.test)


async function deleteCollection(id) {
    const req = await fetch(`http://localhost:3000/collections/${id}`, {
        method: "DELETE"
    });

    const res = await req.json();

    fetch('http://localhost:3000/collections')
        .then(r => r.json())
        .then(r => collections.value = r)
}

</script>
                
<template>
    <div class="component-container">
        <div class="grid grid-cols-3">
            <div class="text-xl text-center inline">Name</div>
            <div class="text-xl text-center inline">Options</div>
            <div class="text-xl text-center inline">Last Updated</div>
        </div>

        <div class="h-4 "></div>
        <div class="h-4/5 overflow-auto">
            <transition-group name="fade" mode="out-in" tag="div">
                <div class="grid grid-cols-3" v-for="collection in collections" :key="collection.id">
                    <div class="text-xl text-center inline link">
                        <router-link :to="{name: 'Collection', params: {id: collection.id}}">{{collection.name}}
                        </router-link>
                    </div>
                    <div class="text-xl text-center inline space-x-5">
                        <font-awesome-icon icon="pen" class="link " />
                        <font-awesome-icon icon="trash" class="link hover:text-red-700"
                            @click="deleteCollection(collection.id)" />
                    </div>
                    <div class="text-xl text-center inline">{{collection.lastUpdated}}</div>
                </div>
            </transition-group>
        </div>

    </div>

</template>
    
<style scoped>

</style>
        