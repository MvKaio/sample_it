<template>
    <div class="AddOrEdit">
        <h3>{{isEdit ? "Edit" : "Add"}}</h3>
        <label>Name:</label>
        <input type="text" v-model="varName"/>
        <br>
        <label>Description:</label>
        <input type="text-area" v-model="varDesc" />
        <br>
        <div class="labels" v-show="varHasLabels">
            <label>Select Labels:</label>
            <select name="labels" v-model="varVal" >
                <option v-for="lb in labels" :key="lb" :value="lb">{{lb}}</option>
            </select>
            <input type="button" @click="addLabel(varVal)" value="Add Label" />
            <ShowLabel v-for="lb in varElmLabels" :key="lb" :name="lb" :deleteFunction="delLabel"></ShowLabel>
        </div>
        <input type="button" @click="isEdit ? edit(id) : add()" :value="isEdit ? 'Edit' : 'Add'">
    </div>
</template>
  
<script>
import ShowLabel from './ShowLabel.vue'
    export default {
    name: "AddOrEdit",
    data() {
        return {
            labels: ["label1", "label2", "label3"],
            varHasLabels: this.hasLabels,
            varIsEdit: this.isEdit,
            varDesc: this.desc,
            varName: this.name,
            varVal: this.val,
            varElmLabels: new Set(this.elmLabels)
        };
    },
    props: {
        hasLabels: {
            type: Boolean,
            default: false
        },
        isEdit: {
            type: Boolean,
            default: false
        },
        id: Number,
        desc: {
            type: String,
            default: ""
        },
        name: {
            type: String,
            default: ""
        },
        val: {
            type: String,
            default: ""
        },
        elmLabels: {
            type: Set,
            default() {
                return new Set;
            }
        }
    },
    methods: {
        edit(obj) {
            // nothing
            console.log(obj);
        },
        add() {
            // nothing
        },
        cancel() {
        },
        addLabel() {
            if(this.varVal == "") 
                return
            if(this.varElmLabels.has(this.varVal))
                return
            this.varElmLabels.add(this.varVal);
            this.varVal = "";
        },
        delLabel(lb){
            this.varElmLabels.delete(lb)
        }
    },
    components: { ShowLabel }
}
</script>
  
  <!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
  