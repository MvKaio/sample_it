<script setup>
import Title from '../components/TitleComponent.vue'
import Header from '../components/HeaderComponent.vue'
import { useRoute } from 'vue-router'
import { onMounted, ref } from 'vue'
import store from '../store';

const route = useRoute()
const collectionID = route.params.id

const collection = ref({})
const collections = ref([])

onMounted(async () => {
    fetch(`http://localhost:3000/collections/${collectionID}`)
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
}

</script>
                    
<template>
    <div class="component-container">
        <h1 class="text-center text-2xl">Collection name: {{collection.name}}</h1>
        <h1 class="text-center text-2xl pt-4">Collection description: </h1>
        <span>{{collection.description}}</span>
        <h1 class="text-center text-2xl">Collection Items:</h1>
        <div v-for="(item, index) in collection.items">{{item.name}}</div>
    </div>
</template>
        
<style scoped>

</style>
            