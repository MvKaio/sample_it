<script setup>
import Title from '../components/TitleComponent.vue'
import Header from '../components/HeaderComponent.vue'
import { onMounted, ref } from 'vue'

const collections = ref([])
onMounted(async () => {
    fetch('http://localhost:3000/collections')
        .then(r => r.json())
        .then(r => collections.value = r)
})


async function deleteCollection(id) {
    const req = await fetch(`http://localhost:3000/collections/${id}`, {
        method: "DELETE"
    });

    const res = await req.json();

    fetch('http://localhost:3000/collections')
        .then(r => r.json())
        .then(r => collections.value = r)
    
    alert("Collection Deleted");
}

</script>
                
<template>
    <div class="flex flex-col text-white max-h-screen h-screen select-none">
        <Title title="Your Collections"></Title>
        <!-- <div class="w-2/5 mx-auto first-letter:items-center"> -->
        <div class="wrapper mx-auto">
            <Header></Header>
            <div class="w-4/5 h-4/5 m-auto pt-4">
                <div class=" bg-purple-400 rounded-lg h-2"></div>
                <div class="grid grid-cols-3 pt-8">
                    <div class="text-xl text-center inline">Name</div>
                    <div class="text-xl text-center inline">Options</div>
                    <div class="text-xl text-center inline">Last Updated</div>
                </div>

                <div class="h-4 "></div>
                <div class="h-4/5 overflow-auto">
                    <div class="grid grid-cols-3" v-for="collection in collections" :key="collection.id">
                        <div class="text-xl text-center inline">{{collection.name}}</div>
                        <div class="text-xl text-center inline space-x-5">
                            <font-awesome-icon icon="pen" class="link " />
                            <font-awesome-icon icon="trash" class="link" @click="deleteCollection(collection.id)" />
                        </div>
                        <div class="text-xl text-center inline">{{collection.lastUpdated}}</div>
                    </div>

                </div>

            </div>
        </div>
    </div>
</template>
    
<style scoped>

</style>
        