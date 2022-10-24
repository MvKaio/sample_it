<script setup>
import { useRoute } from 'vue-router'
import { ref } from 'vue'

const route = useRoute()
const collectionID = route.params.collectionid
const itemID = route.params.itemid
const is_edit = (route.params.edit === "edit")

const collection = ref([])
const item = ref([])
collection.value = await fetch(`http://localhost:3000/collections/${collectionID}`).then((d) => d.json())

item.value = collection.value.items[itemID]

async function deleteItem() {
    collection.value.items.splice(itemID, 1)
    collection.value.updated_at = date

    const dataJson = JSON.stringify(collection.value);
    console.log(dataJson)

    const req = await fetch(`http://localhost:3000/collections/${collectionID}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: dataJson
    });
    /* if (!req.ok) {
        throw new Error(req.status);
    } */
}

async function deleteItemLabel(index) {
    this.labels_editted.splice(index, 1);
}

const editting = ref(false)
const name_editted = ref('')
const description_editted = ref('')
const labels_editted = ref([])
const label_editted = ref('')
const item_label_select = ref('')

function startEdit() {
    name_editted.value = item.value.name
    description_editted.value = item.value.description
    labels_editted.value = Array.from(item.value.labels)
    label_editted.value = ''
    toggleEdit()
}

function cancelEdit() {
    toggleEdit()
}

async function saveEdit() {
    item.value.name = name_editted.value
    item.value.description = description_editted.value
    item.value.labels = Array.from(labels_editted.value)
    item.value.updated_at = date

    collection.value.items[itemID] = item.value

    const dataJson = JSON.stringify(collection.value);
    console.log(dataJson)

    const req = await fetch(`http://localhost:3000/collections/${collectionID}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: dataJson
    });
    /*  if (!req.ok) {
         throw new Error(req.status);
     } */

    toggleEdit()
}

function toggleEdit() {
    editting.value = !editting.value
    console.log(editting.value)
}

function addLabel() {

    if (label_editted.value.length > 0 && !Array.from(labels_editted.value).includes(label_editted.value)) {
        if (!Array.from(collection.value.labels).includes(label_editted.value)) {
            collection.value.labels.push(label_editted.value)
        }
        labels_editted.value.push(label_editted.value)
    }
    else {
        if (Array.from(labels_editted.value).includes(label_editted.value)) {
            alert("Label already exists")
        }
    }
    label_editted.value = ''
}

function addItemLabelSelect() {
    if (item_label_select.value.length > 0 && !Array.from(labels_editted.value).includes(item_label_select.value)) {
        labels_editted.value.push(item_label_select.value)
    }
    else {
        if (Array.from(labels_editted.value).includes(item_label_select.value)) {
            alert("Label already exists")
        }
    }
    item_label_select.value = ''

}

if (is_edit) {
    startEdit()
}

var date = new Date();

</script>
                    
<template>
    <div class="component-container">

        <div class="text-center h-full overflow-auto">
            <transition name="fade" mode="out-in">
                <div v-if="editting" class="h-[5%] flex items-center justify-center">
                    <h1 class="text-center text-2xl">Item name: {{ item.name }}</h1>
                    <input type="text" class="field text-center mx-4" id="name_editted" v-model="name_editted"
                        v-on:keydown.enter.prevent=''>
                </div>
                <div v-else class="h-[5%] flex items-center justify-center">
                    <h1 class="text-center text-2xl">Item name: {{ item.name }}</h1>
                </div>
            </transition>
            <transition name="fade" mode="out-in">
                <div v-if="editting" class="h-[30%] w-[100%] items-center justify-center">
                    <div class="h[20%] flex items-center justify-center">
                        <h1 class="text-center text-2xl">Item description: </h1>
                    </div>
                    <div class="h-[80%]">
                        <textarea type="text" class="textarea_vanilla h-full w-full" id="desription_editted"
                            v-model="description_editted"></textarea>
                    </div>

                </div>
                <div v-else class="h-[30%] w-[100%] items-center justify-center">
                    <div class="h[20%] flex items-center justify-center">
                        <h1 class="text-center text-2xl">Item description: </h1>
                    </div>
                    <div class="h-[80%] black-bg text-left word-break overflow-auto">
                        <span>{{ item.description }}</span>
                    </div>

                </div>
            </transition>

            <transition name="fade" mode="out-in">
                <div v-if="editting" class="h-[55%] w-full flex items-center justify-center">
                    <div class="h-full w-3/5">
                        <div class="w-full flex items-center justify-center">
                            <h1 class="text-center text-2xl">item Labels:</h1>
                        </div>
                        <div class="h-[80%] w-full black-bg overflow-auto">
                            <div class=" mb-4 mx-[1%]" v-for="(lb, index) in labels_editted"><span
                                    class="label-pure-black">{{ lb }}</span>
                                <font-awesome-icon icon="trash" class="link hover:text-red-600 mx-2"
                                    @click="deleteItemLabel(index)" />
                            </div>
                        </div>

                    </div>

                    <div class="h-full w-2/5 flex flex-col items-center justify-center space-y-[5%]">

                        <div class="w-full space-x-[5%]">
                            <select v-model="item_label_select" class="w-[55%]">
                                <option v-for="(lb, index) in collection.labels">{{ lb }}
                                </option>
                            </select>
                            <input class="button w-[35%]" id="addItemLabelButton" type="button" value="Add Label"
                                v-on:click="addItemLabelSelect">
                        </div>

                        <div class="w-full space-x-[5%]">
                            <input type="text" autocomplete="off" class="field w-3/5 text-center" id="label_editted"
                                v-model="label_editted" v-on:keydown.enter.prevent='addLabel'>
                            <input class="button w-[30%]" id="addLabelButton" type="button" value="Add New Label"
                                v-on:click="addLabel">
                        </div>






                    </div>

                </div>

                <div v-else class="h-[55%] w-[100%] items-center justify-center">
                    <div class="flex items-center justify-center">
                        <h1 class="text-center text-2xl">item Labels:</h1>
                    </div>
                    <div class="h-[80%] black-bg overflow-auto">
                        <div class=" mb-4 mx-[1%]" v-for="(lb, index) in item.labels"><span class="label-pure-black">{{
                                lb
                        }}</span></div>
                    </div>
                </div>
            </transition>
            <transition name="fade" mode="out-in">
                <div v-if="editting" class="space-x-4 h-[10%] flex items-center">
                    <div class="text-center w-full space-x-[1%]">
                        <input type="button" class="button" value="Save" @click="saveEdit">
                        <input type="button" class="button" value="Cancel" @click="cancelEdit">
                    </div>
                </div>

                <div v-else class="space-x-4 h-[10%] flex items-center">
                    <div class="text-center w-full space-x-[1%]">
                        <input type="button" class="button" value="Edit Item" @click="startEdit">
                        <input type="button" class="button" value="Delete Item" @click="deleteItem">
                    </div>
                </div>
            </transition>
        </div>
    </div>


</template>
        
<style scoped>

</style>
            