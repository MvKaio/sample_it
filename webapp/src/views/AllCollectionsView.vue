<script setup>

import { ref } from 'vue'

const collections = ref([])
collections.value = await fetch('http://localhost:3000/collections').then((d) => d.json())

async function deleteCollection(id) {
    const req = await fetch(`http://localhost:3000/collections/${id}`, {
        method: "DELETE"
    });

    collections.value = await fetch('http://localhost:3000/collections').then((d) => d.json())
}

function formatDate(dateString) {
    const date = new Date(dateString)
    var dd = String(date.getDate()).padStart(2, '0');
    var mm = String(date.getMonth() + 1).padStart(2, '0');
    var yyyy = date.getFullYear();
    var HH = String(date.getHours()).padStart(2, '0');
    var MM = String(date.getMinutes()).padStart(2, '0');
    var SS = String(date.getSeconds()).padStart(2, '0');
    return dd + '/' + mm + '/' + yyyy + ' ' + HH + ':' + MM;
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

                    <div class="text-xl text-center inline">{{formatDate(collection.updated_at)}}</div>

                    <div class="text-xl text-center inline space-x-5">
                        <!--  -->
                        <router-link :to="{name: 'Collection', params: {id: collection.id, edit: 'edit'}}">
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
        