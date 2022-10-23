<script setup>
import { useRoute } from 'vue-router'
import { ref } from 'vue'

const route = useRoute()
const collectionID = route.params.id
const is_edit = (route.params.edit === "edit")

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

const editting = ref(false)

const name_editted = ref('')
const description_editted = ref('')
const labels_editted = ref([])
const label_editted = ref('')

function startEdit() {
    name_editted.value = collection.value.name
    description_editted.value = collection.value.description
    labels_editted.value = Array.from(collection.value.labels)
    label_editted.value = ''
    toggleEdit()
}

function cancelEdit() {
    toggleEdit()
}

async function saveEdit() {
    collection.value.name = name_editted.value
    collection.value.description = description_editted.value
    collection.value.labels = Array.from(labels_editted.value)
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

    toggleEdit()
}

function toggleEdit() {
    editting.value = !editting.value
    console.log(editting.value)
}

function addLabel() {

    if (label_editted.value.length > 0 && !Array.from(labels_editted.value).includes(label_editted.value)) {
        labels_editted.value.push(label_editted.value)
    }
    else {
        if (Array.from(labels_editted.value).includes(label_editted.value)) {
            alert("Label already exists")
        }
    }
    label_editted.value = ''
}

if (is_edit) {
    startEdit()
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
            <transition name="fade" mode="out-in">
                <div v-if="editting" class="h-[5%] flex items-center justify-center">
                    <h1 class="text-center text-2xl">Collection name: </h1>
                    <input type="text" class="field text-center mx-4" id="name_editted" v-model="name_editted"
                        v-on:keydown.enter.prevent=''>
                </div>
                <div v-else class="h-[5%] flex items-center justify-center">
                    <h1 class="text-center text-2xl">Collection name: {{collection.name}}</h1>

                </div>
            </transition>
            <transition name="fade" mode="out-in">
                <div v-if="editting" class="h-[30%] w-[100%] items-center justify-center">
                    <div class="h[20%] flex items-center justify-center">
                        <h1 class="text-center text-2xl">Collection description: </h1>
                    </div>
                    <div class="h-[80%]">
                        <textarea type="text" class="textarea_vanilla h-full w-full" id="desription_editted"
                            v-model="description_editted"></textarea>
                    </div>

                </div>
                <div v-else class="h-[30%] w-[100%] items-center justify-center">
                    <div class="h[20%] flex items-center justify-center">
                        <h1 class="text-center text-2xl">Collection description: </h1>
                    </div>
                    <div class="h-[80%] black-bg text-left word-break overflow-auto">
                        <span>{{collection.description}}</span>
                    </div>

                </div>
            </transition>


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
                            <router-link :to="{name: 'View Item', params: {collectionid: collection.id, itemid: index}}"
                                class="link">
                                {{item.name}}
                            </router-link>
                        </div>

                        <div class="text-xl text-center whitespace-pre space-x-[2%] p-2 overflow-auto">
                            <span class="label-pure-black" v-for="(lb, index) in item.labels">{{lb}}</span>
                        </div>

                        <div class="text-xl text-center space-x-[5%] label-pure-black flex items-center justify-center">
                            <router-link
                                :to="{name: 'View Item', params: {collectionid: collection.id, itemid: index, edit: 'edit'}}">
                                <font-awesome-icon icon="pen" class="link " />
                            </router-link>
                            <font-awesome-icon icon="trash" class="link hover:text-red-700"
                                @click="deleteItem(index)" />
                        </div>

                    </div>
                </div>
            </div>

            <div v-if="editting" class="h-[25%] w-full flex items-center justify-center">
                <div class="h-full w-3/5">
                    <div class="w-full flex items-center justify-center">
                        <h1 class="text-center text-2xl">Collection Labels:</h1>
                    </div>
                    <div class="h-[80%] w-full black-bg overflow-auto">
                        <div class=" mb-4 mx-[1%]" v-for="(lb, index) in labels_editted"><span
                                class="label-pure-black">{{lb}}</span></div>
                    </div>

                </div>

                <div class="h-full w-2/5 flex items-center justify-center space-x-[5%]">
                    <input type="text" class="field w-3/5 text-center" id="label_editted" v-model="label_editted"
                        v-on:keydown.enter.prevent='addLabel'>
                    <input class="button w-[30%]" id="addLabelButton" type="button" value="Add Label"
                        v-on:click="addLabel">

                </div>

            </div>

            <div v-else class="h-[25%] w-[100%] items-center justify-center">
                <div class="flex items-center justify-center">
                    <h1 class="text-center text-2xl">Collection Labels:</h1>
                </div>
                <div class="h-[80%] black-bg overflow-auto">
                    <div class=" mb-4 mx-[1%]" v-for="(lb, index) in collection.labels"><span
                            class="label-pure-black">{{lb}}</span></div>
                </div>
            </div>

            <div v-if="editting" class="space-x-4 h-[10%] flex items-center">
                <div class="text-center w-full space-x-[1%]">
                    <input type="button" class="button" value="Save" @click="saveEdit">
                    <input type="button" class="button" value="Cancel" @click="cancelEdit">
                </div>
            </div>
            <div v-else class="space-x-4 h-[10%] flex items-center">
                <div class="text-center w-full space-x-[1%]">
                    <router-link :to="{name: 'Generate Sample', params: {id: collectionID}}"><input type="button"
                            class="button" value="Generate a New Sample"></router-link>
                    <router-link :to="{name: 'Add Item', params: {id: collectionID}}"><input type="button"
                            class="button" value="Add Items"></router-link>
                    <input type="button" class="button" value="Edit Collection" @click="startEdit">
                    <input type="button" class="button" value="Delete Collection" @click="deleteCollection">
                </div>
            </div>
        </div>
    </div>


</template>
        
<style scoped>

</style>
            