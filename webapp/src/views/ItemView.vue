<script setup>
import { useRoute } from 'vue-router'
import { ref } from 'vue'

const route = useRoute()
const collectionID = route.params.collectionid
const itemID = route.params.itemid

const collection = ref([])
const item = ref([])
collection.value = await fetch(`http://localhost:3000/collections/${collectionID}/`).then((d) => d.json())

item.value = collection.value.items[itemID]

/* async function deleteCollection() {
    const req = await fetch(`http://localhost:3000/collections/${collectionID}`, {
        method: "DELETE"
    });

    const res = await req.json();

    fetch('http://localhost:3000/collections')
        .then(r => r.json())
        .then(r => collections.value = r)


    document.getElementById('allCollectionsLink').click();
} */

</script>
                    
<template>
    <div class="component-container">

        <div class="text-center h-full overflow-auto">
            <div class="h-[5%] flex items-center justify-center">
                <h1 class="text-center text-2xl">Item name: {{item.name}}</h1>
            </div>
            <div class="h-[30%] w-[100%] items-center justify-center">
                <div class="h[20%] flex items-center justify-center">
                    <h1 class="text-center text-2xl">Item description: </h1>
                </div>
                <div class="h-[80%] black-bg text-left word-break overflow-auto">
                    <span>{{collection.description}}</span>
                </div>

            </div>

            <div class="h-[55%] w-[100%] items-center justify-center">
                <div class="h[20%] flex items-center justify-center">
                    <h1 class="text-center text-2xl">Item Labels:</h1>
                </div>
                <div class="h-[80%] black-bg overflow-auto">
                    <div v-for="(lb, index) in item.labels">{{lb}}</div>
                </div>
            </div>

            <div class="space-x-4 h-[10%] flex items-center">
                <div class="text-center w-full space-x-[1%]">
                    <router-link :to="{name: 'Generate Sample', params: {id: collectionID}}"><input type="button"
                            class="button" value="Generate a New Sample"></router-link>
                    <router-link :to="{name: 'EditCollection', params: {id: collectionID}}"><input type="button"
                            class="button" value="Edit"></router-link>
                    <input type="button" class="button" value="Delete this item" @click="deleteCollection">
                </div>
            </div>
        </div>
    </div>


</template>
        
<style scoped>

</style>
            