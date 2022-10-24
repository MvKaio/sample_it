<script setup>
import { ref } from 'vue'

const collections = ref([])

collections.value = await fetch('http://localhost:3000/collections').then((d) => d.json())

function last(array) {
    return array[array.length - 1];
}

function clearItemFields() {
    item_name.value = ''
    item_description.value = ''
    item_label.value = ''
    item_labels.value = []
    item_label_select.value = ''
}

function clearCollectionFields() {
    label.value = ''
    labels.value = []
    name.value = ''
    description.value = ''
    items.value = []

}

// Cria a coleção se todos os campos estão preenchidos. Envia os dados em "data" pelo método "POST"
async function createCollection() {

    if (name.value.length > 0 && description.value.length > 0 && Array.from(labels.value).length > 0) {
        const data = {
            name: name.value,
            description: description.value,
            labels: Array.from(labels.value),
            updated_at: date,
            created_at: date,
            items: items.value,
        }

        const dataJson = JSON.stringify(data);

        await fetch("http://localhost:3000/collections", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: dataJson
        });

        /*  if (!req.ok) {
             throw new Error(req.status);
         } */
        if (items.value.length == 0) {
            alert("Empty collection created")
        }
        clearCollectionFields();
        clearItemFields();
        form_item.value = false;
    }
    else {
        alert("Please fill all the inputs")
    }
}

async function addFirstItemFunction() {
    if (name.value.length > 0 && description.value.length > 0 && Array.from(labels.value).length > 0) {
        form_item.value = true
    }
    else {
        alert("Please fill all the inputs")
    }
}

async function addItemFunction() {
    if (item_name.value.length > 0 && item_description.value.length > 0 && Array.from(item_labels.value).length > 0) {
        const item = {
            name: item_name.value,
            description: item_description.value,
            labels: item_labels.value,
        }
        items.value.push(item)
        console.log(items.value)
        clearItemFields()
    }
    else {
        alert("Please fill all the inputs")
    }

}

//Função que é chamada ao clicar em adicionar label. Adiciona o label no array labels se o label ainda não existe
async function addLabel() {

    if (label.value.length > 0 && !Array.from(labels.value).includes(label.value)) {
        labels.value.push(label.value)
    }
    else {
        if (Array.from(labels.value).includes(label.value)) {
            alert("Label already exists")
        }
    }
    label.value = ''
}

async function addItemLabel() {

    if (item_label.value.length > 0 && !Array.from(item_labels.value).includes(item_label.value)) {
        if (!Array.from(labels.value).includes(item_label.value)) {
            labels.value.push(item_label.value)
        }
        item_labels.value.push(item_label.value)
    }
    else {
        if (Array.from(item_labels.value).includes(item_label.value)) {
            alert("Item Label already exists")
        }
    }
    item_label.value = ''
}


async function addItemLabelSelect() {

    if (item_label_select.value.length > 0 && !Array.from(item_labels.value).includes(item_label_select.value)) {
        item_labels.value.push(item_label_select.value)
    }
    else {
        if (Array.from(item_labels.value).includes(item_label_select.value)) {
            alert("ItemLabel already exists")
        }
    }
    item_label_select.value = ''
}

//Função que deleta label a partir do seu índice
function deleteLabel(index) {
    this.labels.splice(index, 1);
    console.log(labels.value)
}

function deleteItemLabel(index) {
    this.item_labels.splice(index, 1);
    console.log(item_labels.value)
}

//Definindo a data de alteração da coleção
var date = new Date();
/* var dd = String(date.getDate()).padStart(2, '0');
var mm = String(date.getMonth() + 1).padStart(2, '0');
var yyyy = date.getFullYear();
date = dd + '/' + mm + '/' + yyyy; */

//Refs para name, description, label, labels e items da collection
const name = ref("")
const description = ref("")
const label = ref("")
const labels = ref([])
const items = ref([])

//Refs para name, description, label e labels da collection

const item_name = ref("")
const item_description = ref("")
const item_label_select = ref("")
const item_label = ref("")
const item_labels = ref([])

const form_item = ref(false)


</script>
            
<template>
    <transition name="fade" mode="out-in">
        <div v-if="!form_item" class="component-container">
            <div class="form h-full text-center">
                <form v-on:submit.prevent class="h-full" @submit="" autocomplete="off">
                    <div class="input-group h-[10%] py-[1%]">
                        <div class="text-2xl">
                            <label for="name">Name</label>
                        </div>

                        <input type="text" class="field w-3/5 text-center" id="name" v-model="name"
                            v-on:keydown.enter.prevent=''>
                    </div>
                    <div class="input-group h-[30%] py-[1%]">
                        <div class="text-2xl h-[20%]">
                            <label for="description">Description</label>
                        </div>
                        <textarea type="text" class="textarea" id="desription" v-model="description"></textarea>
                    </div>

                    <div class="input-group h-[10%] w-3/5 m-auto space-x-[2%] py-[1%]">
                        <input type="text" class="field w-3/5 text-center" id="label" v-model="label"
                            v-on:keydown.enter.prevent='addLabel'>
                        <input class="button w-[38%]" id="addLabelButton" type="button" value="Add Label"
                            v-on:click="addLabel">
                    </div>
                    <div class="flex flex-col pt-4 space-y-4 h-[40%] w-3/5 m-auto overflow-auto">
                        <div v-for="(lb, index) in labels" class="inline">
                            <span class="label">{{ lb }}</span>
                            <font-awesome-icon icon="trash" class="link hover:text-red-600 mx-2"
                                @click="deleteLabel(index)" />
                        </div>
                    </div>
                    <div class="space-x-4 h-[10%] flex items-center justify-center  ">
                        <input type="button" class="button" @click="createCollection" value="Create Empty Collection">
                        <input type="button" class="button" @click="addFirstItemFunction" value="Add an Item">
                    </div>

                </form>
            </div>
        </div>
        <!-- v-else para renderizar o template de adicionar item -->
        <div v-else class="component-container">
            <div class="h-full grid grid-cols-2 overflow-auto">
                <!-- div dados collection -->
                <div class="text-center h-full overflow-auto">
                    <div class="h-[5%] flex items-center justify-center">
                        <h1 class="text-center text-2xl">Collection name: {{ name }}</h1>

                    </div>
                    <div class="h-[30%] w-[100%] items-center justify-center">
                        <div class="h[20%] flex items-center justify-center">
                            <h1 class="text-center text-2xl">Collection description: </h1>
                        </div>
                        <div class="h-[80%] black-bg text-left word-break overflow-auto">
                            <span>{{ description }}</span>
                        </div>

                    </div>

                    <div class="h-[30%] w-[100%] items-center justify-center">
                        <div class="h[20%] flex items-center justify-center">
                            <h1 class="text-center text-2xl">Collection Items:</h1>
                        </div>
                        <div class="h-[80%] black-bg overflow-auto">
                            <div class="mb-4" v-for="(item, index) in items"><span class="label-pure-black">{{ item.name
                            }}</span>
                            </div>
                        </div>
                    </div>

                    <div class="h-[25%] w-[100%] items-center justify-center">
                        <div class="h[20%] flex items-center justify-center">
                            <h1 class="text-center text-2xl">Collection Labels:</h1>
                        </div>
                        <div class="h-[80%] black-bg overflow-auto">
                            <div class=" mb-4" v-for="(lb, index) in labels"><span class="label-pure-black">{{ lb
                            }}</span>
                            </div>
                        </div>
                    </div>

                    <div class="space-x-4 h-[10%] flex items-center">
                        <div class="text-center w-full">
                            <input type="button" class="button" @click="createCollection" value="Create Collection">
                        </div>
                    </div>
                </div>
                <div class="text-center h-full">
                    <form v-on:submit.prevent class="h-full" @submit="" autocomplete="off">
                        <div class="input-group h-[10%] w-[60%] m-auto">
                            <div class="text-2xl">
                                <label for="name">Item Name</label>
                            </div>

                            <input type="text" class="field w-[100%] text-center" id="name" v-model="item_name">
                        </div>
                        <div class="input-group h-[30%] overflow-auto">
                            <div class="text-2xl">
                                <label for="description">Item Description</label>
                            </div>

                            <textarea type="text" class="textarea" id="desription"
                                v-model="item_description"></textarea>
                        </div>
                        <div class="input-group h-[10%] w-[60%] m-auto space-x-[5%]">
                            <input type="text" class="field w-[60%] text-center" id="label" v-model="item_label"
                                v-on:keydown.enter.prevent='addItemLabel'>
                            <input class="button w-[35%]" id="addItemLabelButton" type="button" value="Add New Label"
                                v-on:click="addItemLabel">
                        </div>
                        <div class="input-group h-[10%] w-[60%] m-auto space-x-[5%]">
                            <select v-model="item_label_select" class="w-[60%]">
                                <option v-for="(lb, index) in labels">{{ lb }}
                                </option>
                            </select>
                            <input class="button w-[35%]" id="addItemLabelButton" type="button" value="Add Item Label"
                                v-on:click="addItemLabelSelect">

                        </div>
                        <div class="flex flex-col pt-4 space-y-4 h-[30%] w-3/5 m-auto overflow-auto">
                            <div v-for="(lb, index) in item_labels" class="inline">
                                <span class="label">{{ lb }}</span>
                                <font-awesome-icon icon="trash" class="link hover:text-red-600 mx-2"
                                    @click="deleteItemLabel(index)" />
                            </div>
                        </div>
                        <div class="space-x-4 h-[10%] flex items-center">
                            <div class="text-center w-full">
                                <input type="button" class="button" @click="addItemFunction" value="Add this Item">
                            </div>


                        </div>

                    </form>

                </div>

            </div>
        </div>
    </transition>
</template>

<style scoped>
.field {
    background: #212121;
    border-bottom: 4px dashed var(--primary-color);
    border-radius: 5px;
    outline: none;
    height: 1.9rem;
    padding: 10px;
}

.textarea {

    background: #212121;
    border-bottom: 4px dashed var(--primary-color);
    border-radius: 5px;
    outline: none;
    height: 80%;
    padding: 10px;
    width: 60%;
    resize: none;
}

select {
    background: #212121;
    border-bottom: 4px dashed var(--primary-color);
    border-radius: 5px;
    outline: none;
    padding: 10px;
}

.label {
    background: #212121;
    border-bottom: 2px solid var(--primary-color);
    border-radius: 5px;
    outline: none;
    padding: 5px;
}

.input-group {
    align-items: center;
}



.word-break {
    word-wrap: break-word;
}
</style>
    