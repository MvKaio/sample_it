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


async function createCollection(e) {

    e.preventDefault();

    if (name.value.length > 0 && description.value.length > 0 && Array.from(labels.value).length > 0) {
        const data = {
            name: name.value,
            description: description.value,
            labels: Array.from(labels.value),
            lastUpdated: today,
            items: []

        }

        const dataJson = JSON.stringify(data);

        const req = await fetch("http://localhost:3000/collections", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: dataJson
        });

        const res = await req.json();
        //console.log(res);

        //console.log(data)
        alert("Collection Created");
    }



}

async function addLabel() {

    if (label.value.length > 0 && !Array.from(labels.value).includes(label.value)) {
        labels.value.push(label.value)
        //console.log(Array.from(labels.value))
    }
    label.value = ''
}

function deleteLabel(index) {
    this.labels.splice(index, 1);
    console.log(labels.value)
}

var today = new Date();
var dd = String(today.getDate()).padStart(2, '0');
var mm = String(today.getMonth() + 1).padStart(2, '0');
var yyyy = today.getFullYear();

today = dd + '/' + mm + '/' + yyyy;

const name = ref("")
const description = ref("")
const label = ref("")
const labels = ref([])

</script>
            
<template>
    <div class="flex flex-col text-white max-h-screen h-screen select-none">
        <Title title="Create Your New Collection!"></Title>
        <!-- <div class="w-2/5 mx-auto first-letter:items-center"> -->
        <div class="wrapper mx-auto">

            <Header></Header>

            <div class="w-4/5 h-4/5 m-auto pt-4">

                <div class=" bg-purple-400 rounded-lg h-2"></div>
                <div class="form text-center">
                    <form v-on:submit.prevent class="space-y-3" @submit="createCollection">
                        <div class="input-group">
                            <div>
                                <label for="name">Name</label>
                            </div>

                            <input type="text" class="field" id="name" v-model="name">
                        </div>
                        <div class="input-group">
                            <div>
                                <label for="description">Description</label>
                            </div>

                            <textarea type="text" style="height: 202px" class=" field" id="desription"
                                v-model="description"></textarea>
                        </div>
                        <div class="input-group space-x-4">
                            <input type="text" class="field" id="label" v-model="label"
                                v-on:keydown.enter.prevent='addLabel'>
                            <input class="button" id="addLabelButton" type="button" value="Add Label"
                                v-on:click="addLabel">
                        </div>
                        <div class="flex flex-col space-y-4 h-40 w-3/5 m-auto overflow-auto">
                            <div v-for="(lb, index) in labels" class="inline">
                                <span class="label">{{lb}}</span>
                                <font-awesome-icon icon="trash" class="link hover:text-red-600 mx-2"
                                    @click="deleteLabel(index)" />
                            </div>
                        </div>
                        <div class="space-x-4">
                            <input class="button" type="submit" value="Create Collection">
                        </div>

                    </form>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.field {
    background: #212121;
    border-bottom: 4px dashed purple;
    border-radius: 5px;
    outline: none;
    height: 1.9rem;
    padding: 10px;
}

.label {
    background: #212121;
    border-bottom: 2px solid purple;
    border-radius: 5px;
    outline: none;
    padding: 5px;
}

.input-group {
    align-items: center;
}

.form {
    margin-top: 5%;
}

textarea {
    width: 60%;
    resize: none;
}
</style>
    