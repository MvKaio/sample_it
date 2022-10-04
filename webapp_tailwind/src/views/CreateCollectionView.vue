<script setup>
import Title from '../components/TitleComponent.vue'
import { onMounted, ref } from 'vue'

const collections = ref([])
onMounted(async () => {
    fetch('http://localhost:3000/collections')
        .then(r => r.json())
        .then(r => collections.value = r)
})
</script>
            
<template>
    <div class="flex flex-col text-white max-h-screen h-screen select-none">
        <Title title="Create Your New Collection!"></Title>
        <!-- <div class="w-2/5 mx-auto first-letter:items-center"> -->
        <div class="wrapper mx-auto">

            <div class="grid grid-cols-2 pt-8">
                <div class="text-2xl px-8 text-right inline">
                    <router-link to="/" class="link">Home</router-link>
                </div>
                <div class="text-2xl text-left inline">
                    <router-link to="/" class="link">Save</router-link>
                </div>
            </div>

            <div class="w-4/5 h-4/5 m-auto pt-4">

                <div class=" bg-purple-400 rounded-lg h-2"></div>
                <div class="form text-center p-4">
                    <form class="space-y-5" action="">
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
                        <button class="button" type="submit">Create Collection</button>
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

.input-group {
    align-items: center;
}

.form {
    margin-top: 10%;
}

textarea {
    width: 60%;
    resize: none;
}
</style>
    