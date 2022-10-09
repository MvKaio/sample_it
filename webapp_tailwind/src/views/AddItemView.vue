<script setup>
import Title from '../components/TitleComponent.vue'
import Header from '../components/HeaderComponent.vue'
import { watch, onMounted, ref } from 'vue'

const collections = ref([])
onMounted(async () => {
    fetch('http://localhost:3000/collections')
        .then(r => r.json())
        .then(r => collections.value = r)
})

const collection = ref([])

</script>
                    
<template>
    <div class="component-container">
        <!-- <p>{{collection.name}}</p> -->

        <div class=" bg-purple-400 rounded-lg h-2"></div>
        <div class="text-center">
            <h1></h1>
        </div>

        <div class="form text-center p-4">
            <form class="space-y-5">
                <div class="input-group">
                    <div>
                        <label for="name">Name</label>
                    </div>

                    <input type="text" class="field" id="name">
                </div>
                <div class="input-group">
                    <div>
                        <label for="description">Description</label>
                    </div>
                    <textarea type="text" style="height: 202px" class=" field" id="desription"></textarea>
                </div>
                <div class="input-group">
                    <div>
                        <label for="labels">Labels</label>
                    </div>
                    <select>
                        <option>teste</option>
                        <option>teste2</option>
                    </select>
                    <div class="inline px-4">
                        <button class="button">Add Label</button>
                    </div>
                    <div class="py-4">
                        <input type="text" class="field w-2/5" id="label">
                        <div class="inline px-4">
                            <button class="button">Add Custom Label</button>
                        </div>
                    </div>
                </div>
                <div class="labels">

                </div>
                <span class="button">Add Item</span>
            </form>
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

.input-group {
    align-items: center;
}

.form {
    margin-top: 2%;
}

.labels {
    height: 1.4rem;
}

textarea {
    width: 60%;
    resize: none;
}

select {
    background: #212121;
    border-bottom: 4px solid purple;
    border-radius: 5px;
    outline: none;
    padding: 10px;
    width: 40%;
}
</style>
            