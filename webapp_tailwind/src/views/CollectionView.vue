<script setup>
import { useRoute } from 'vue-router'
import { onMounted, ref } from 'vue'
import store from '../store';
import router from '../router';

const route = useRoute()
const collectionID = route.params.id
const id = ref(0)
id.value = collectionID

const collection = ref({})
const collections = ref([])

onMounted(async () => {
    fetch(`http://localhost:3000/collections/${collectionID}/`)
        .then(r => r.json())
        .then(r => collection.value = r)

})

async function deleteCollection() {
    const req = await fetch(`http://localhost:3000/collections/${collectionID}`, {
        method: "DELETE"
    });

    const res = await req.json();

    fetch('http://localhost:3000/collections')
        .then(r => r.json())
        .then(r => collections.value = r)


    document.getElementById('allCollectionsLink').click();
}

</script>
                    
<template>
    <div class="component-container">
        <!-- <h1 class="text-center text-2xl">Collection name: {{collection.name}}</h1>
        <h1 class="text-center text-2xl pt-4">Collection description: </h1>
        <span>{{collection.description}}</span>
        <h1 class="text-center text-2xl">Collection Items:</h1>
        <div v-for="(item, index) in collection.items">{{item.name}}</div> -->

        <div class="text-center h-full overflow-auto">
            <div class="h-[5%] flex items-center justify-center">
                <h1 class="text-center text-2xl">Collection name: {{collection.name}}</h1>

            </div>
            <div class="h-[30%] w-[100%] items-center justify-center">
                <div class="h[20%] flex items-center justify-center">
                    <h1 class="text-center text-2xl">Collection description: </h1>
                </div>
                <div class="h-[80%] black-bg text-left word-break overflow-auto">
                    <span>{{collection.description}}</span>
                </div>

            </div>

            <div class="h-[30%] w-[100%] items-center justify-center">
                <div class="h[20%] flex items-center justify-center">
                    <h1 class="text-center text-2xl">Collection Items:</h1>
                </div>
                <div class="h-[80%] black-bg overflow-auto">
                    <div v-for="(item, index) in collection.items">{{item.name}}</div>
                </div>
            </div>

            <div class="h-[25%] w-[100%] items-center justify-center">
                <div class="h[20%] flex items-center justify-center">
                    <h1 class="text-center text-2xl">Collection Labels:</h1>
                </div>
                <div class="h-[80%] black-bg overflow-auto">
                    <div v-for="(lb, index) in collection.labels">{{lb}}</div>
                </div>
            </div>

            <div class="space-x-4 h-[10%] flex items-center">
                <div class="text-center w-full space-x-[1%]">
                    <router-link :to="{name: 'Generate Sample', params: {id: route.params.id}}"><input type="button"
                            class="button" value="Generate a New Sample"></router-link>
                    <input type="button" class="button" value="Edit this collection">
                    <input type="button" class="button" value="Delete this collection" @click="deleteCollection">
                </div>
            </div>
        </div>
    </div>


</template>
        
<style scoped>

</style>
            