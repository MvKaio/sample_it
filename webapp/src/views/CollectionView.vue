<script setup>
import { useRoute } from 'vue-router'
import { ref } from 'vue'

const route = useRoute()
const collectionID = route.params.id

const collections = ref([])
const collection = ref([])

collection.value = await fetch(`http://localhost:3000/collections/${collectionID}/`).then((d) => d.json())

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

async function deleteItem(index) {
    collection.value.items.splice(index, 1)
    collection.value.updated_at = date

    const dataJson = JSON.stringify(collection.value);
    console.log(dataJson)

    const req = await fetch(`http://localhost:3000/collections/${collectionID}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: dataJson
    });
    if (!req.ok) {
        throw new Error(req.status);
    }
}

//Definindo a data de alteração da coleção
var date = new Date();

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
                <div class="h-[20%] flex items-center justify-center">
                    <h1 class="text-center text-2xl">Collection Items:</h1>
                </div>
                <div class="grid grid-cols-3 h-[20%]">
                    <div class="text-2xl flex items-center justify-center">Name</div>
                    <div class="text-2xl flex items-center justify-center">Labels</div>
                    <div class="text-2xl flex items-center justify-center">Options</div>
                </div>
                <div class="h-[60%] black-bg overflow-auto">
                    <div class="grid grid-cols-3 mb-4 mx-[1%] space-x-[10%]" v-for="(item, index) in collection.items">
                        <div class="text-xl text-centerlink label-pure-black flex items-center justify-center">
                            <router-link
                                :to="{name: 'View Item', params: {collectionid: collection.id, itemid: index}}" class="link">
                                {{item.name}}
                            </router-link>
                        </div>

                        <div class="text-xl text-center whitespace-pre space-x-[2%] p-2 overflow-auto">
                            <span class="label-pure-black" v-for="(lb, index) in item.labels">{{lb}}</span>
                        </div>

                        <div class="text-xl text-center space-x-[5%] label-pure-black flex items-center justify-center">

                            <!-- <router-link :to="{name: 'EditItem', params: {collectionid: collection.id, itemid: index}}">
                                <font-awesome-icon icon="pen" class="link " />
                            </router-link> -->
                            <font-awesome-icon icon="trash" class="link hover:text-red-700"
                                @click="deleteItem(index)" />
                        </div>

                    </div>
                </div>
            </div>

            <div class="h-[25%] w-[100%] items-center justify-center">
                <div class="h[20%] flex items-center justify-center">
                    <h1 class="text-center text-2xl">Collection Labels:</h1>
                </div>
                <div class="h-[80%] black-bg overflow-auto">
                    <div class=" mb-4 mx-[1%]" v-for="(lb, index) in collection.labels"><span
                            class="label-pure-black">{{lb}}</span></div>
                </div>
            </div>

            <div class="space-x-4 h-[10%] flex items-center">
                <div class="text-center w-full space-x-[1%]">
                    <router-link :to="{name: 'Generate Sample', params: {id: collectionID}}"><input type="button"
                            class="button" value="Generate a New Sample"></router-link>
                    <router-link :to="{name: 'EditCollection', params: {id: collectionID}}"><input type="button"
                            class="button" value="Edit"></router-link>
                    <router-link :to="{name: 'Add Item', params: {id: collectionID}}"><input type="button"
                            class="button" value="Add Items"></router-link>
                    <input type="button" class="button" value="Delete this collection" @click="deleteCollection">
                </div>
            </div>
        </div>
    </div>


</template>
        
<style scoped>

</style>
            