<script setup>
import Title from '../components/TitleComponent.vue'
import Header from '../components/HeaderComponent.vue'
import { useRoute } from 'vue-router'
import { onMounted, ref } from 'vue'
import store from '../store';

const route = useRoute()
const collectionID = route.params.id

const collection = ref({})
const collections = ref([])

onMounted(async () => {
    fetch(`http://localhost:3000/collections/${collectionID}`)
        .then(r => r.json())
        .then(r => {
            collection.value = r
        })
})

function clearConstraint() {
    constraint_operator.value = '';
    constraint_number.value = 0;
    constraint_label.value = '';
}

async function addConstraint() {

    const constraint = {
        operator: constraint_operator.value,
        number: constraint_number.value,
        label: constraint_label.value,
    }

    if (constraint.operator.length > 0 && constraint.number > 0 && constraint.label.length > 0) {
        constraints.value.push(constraint);
        clearConstraint();
    }

}

function deleteConstraint(index) {
    this.constraints.splice(index, 1);

}

const sample_name = ref("")
const sample_description = ref("")

const constraint_operator = ref('');
const constraint_number = ref(0);
const constraint_label = ref('')

const constraints = ref([])


</script>
                    
<template>
    <div class="component-container">
        <!-- <h1 class="text-center text-2xl">Collection name: {{collection.name}}</h1>
        <h1 class="text-center text-2xl pt-4">Collection description: </h1>
        <span>{{collection.description}}</span>
        <h1 class="text-center text-2xl">Collection Items:</h1>
        <div v-for="(item, index) in collection.items">{{item.name}}</div> -->

        <div class="h-full grid grid-cols-2 overflow-auto">
            <!-- div dados collection -->
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
                    <div class="h[20%] flex items-center justify-center">
                        <h1 class="text-center text-2xl">Collection Items:</h1>
                    </div>
                    <div class="h-[80%] black-bg overflow-auto">
                        <div v-for="(item, index) in collection.items">{{item.name}}</div>
                    </div>
                </div>

                <div class="h-[25%] w-[100%] items-center justify-center">
                    <div class="h[20%] flex items-center justify-center">
                        <h1 class="text-center text-2xl">Collection Labels:</h1>
                    </div>
                    <div class="h-[80%] black-bg overflow-auto">
                        <div v-for="(lb, index) in collection.labels">{{lb}}</div>
                    </div>
                </div>

                <div class="space-x-4 h-[10%] flex items-center">
                    <div class="text-center w-full">
                        <input type="button" class="button" value="Edit Collection">
                    </div>
                </div>
            </div>
            <div class="text-center h-full">
                <form v-on:submit.prevent class="h-full" @submit="" autocomplete="off">
                    <div class="input-group h-[10%] w-[60%] m-auto">
                        <div class="text-2xl">
                            <label for="name">Sample Name</label>
                        </div>

                        <input type="text" class="field w-[100%] text-center" id="name" v-model="sample_name">
                    </div>
                    <div class="input-group h-[30%] overflow-auto">
                        <div class="text-2xl">
                            <label for="description">Sample Description</label>
                        </div>

                        <textarea type="text" class="textarea" id="desription" v-model="sample_description"></textarea>
                    </div>

                    <div class="input-group h-[15%]">
                        <div class="text-2xl h-2/5">
                            <span>Sample Constraints</span>
                        </div>
                        <div class="h-3/5 w-[60%] m-auto space-x-[5%]">
                            <!-- <input type="text" class="field w-[85%] text-center" id="label" v-model="item_label"
                                v-on:keydown.enter.prevent='addItemLabel'> -->

                            <div class="w-[85%] inline-block space-x-[3%]">

                                <select v-model="constraint_label" class="w-[50%]">
                                    <option v-for="(lb, index) in collection.labels">{{lb}}
                                    </option>
                                </select>

                                <select v-model="constraint_operator" class="w-[17%]">
                                    <option value="equal">=</option>
                                    <option value="less_or_equal">≤</option>
                                    <option value="more_or_equal">≥</option>
                                </select>

                                <input v-model="constraint_number" class="w-[23%] number" type="number" min="0">


                            </div>


                            <input class="button w-[10%]" id="addConstraintButton" type="button" value="+"
                                v-on:click="addConstraint">
                        </div>

                    </div>
                    <div class="flex flex-col pt-4 space-y-4 h-[35%] w-3/5 m-auto overflow-auto">
                        <div v-for="(constraint, index) in constraints" class="inline">
                            <div v-if="constraint.operator == 'equal'">
                                <span class="label">Exactly {{constraint.number}} item(s) with label
                                    {{constraint.label}}</span>
                                <font-awesome-icon icon="trash" class="link hover:text-red-600 mx-2"
                                    @click="deleteConstraint(index)" />

                            </div>

                            <div v-if="constraint.operator == 'less_or_equal'">
                                <span class="label">At most {{constraint.number}} item(s) with label
                                    {{constraint.label}}</span>
                                <font-awesome-icon icon="trash" class="link hover:text-red-600 mx-2"
                                    @click="deleteConstraint(index)" />

                            </div>

                            <div v-if="constraint.operator == 'more_or_equal'">
                                <span class="label">At least {{constraint.number}} item(s) with label
                                    {{constraint.label}}</span>
                                <font-awesome-icon icon="trash" class="link hover:text-red-600 mx-2"
                                    @click="deleteConstraint(index)" />

                            </div>

                        </div>
                    </div>
                    <div class="space-x-4 h-[10%] flex items-center">
                        <div class="text-center w-full">
                            <input type="button" class="button" @click="generateSample" value="Generate Sample">
                        </div>


                    </div>

                </form>

            </div>

        </div>
    </div>


</template>
        
<style scoped>

</style>
            