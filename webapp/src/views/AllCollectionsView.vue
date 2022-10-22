<script setup>

import { ref } from 'vue'

const collections = ref([])
collections.value = await fetch('http://localhost:3000/collections').then((d) => d.json())

async function deleteCollection(id) {
    const req = await fetch(`http://localhost:3000/collections/${id}`, {
        method: "DELETE"
    });

    const res = await req.json();

    collections.value = await fetch('http://localhost:3000/collections').then((d) => d.json())
}

</script>
                
<template>
    <div class="component-container">
        <div class="grid grid-cols-3 h-[8%]">
            <div class="text-2xl flex items-center justify-center">Name</div>
            <div class="text-2xl flex items-center justify-center">Last Updated</div>
            <div class="text-2xl flex items-center justify-center">Options</div>
        </div>
        <div class="h-[82%] overflow-auto">
            <transition-group name="fade" mode="out-in" tag="div">
                <div class="grid grid-cols-3 black-bg mb-4 mx-[1%]" v-for="collection in collections"
                    :key="collection.id">
                    <div class="text-xl text-center inline link">
                        <router-link :to="{name: 'Collection', params: {id: collection.id}}">{{collection.name}}
                        </router-link>
                    </div>

                    <div class="text-xl text-center inline">{{collection.updated_at}}</div>

                    <div class="text-xl text-center inline space-x-5">
                        <!--  -->
                        <router-link :to="{name: 'EditCollection', params: {id: collection.id}}">
                            <font-awesome-icon icon="pen" class="link " />
                        </router-link>
                        <font-awesome-icon icon="trash" class="link hover:text-red-700"
                            @click="deleteCollection(collection.id)" />
                    </div>
                </div>
            </transition-group>
        </div>
        <div class="space-x-4 h-[10%] flex items-center">
            <div class="text-center w-full space-x-[1%]">
                <router-link to="/createCollection" class="link"><input type="button" class="button"
                        value="Create a New Collection"></router-link>
            </div>
        </div>

    </div>

</template>
    
<style scoped>

</style>
        