<template>
  <div class="ListElements">
    <h2>{{title}}</h2>
    <AddOrEdit :hasLabels="true" :isEdit="isEdit" :addFunction="addElm" :editFunction="editElm" :desc="desc" :elmLabels="elmLabels" :setIsEdit="setIsEdit" :name="name"></AddOrEdit>
    <AppElement v-for="name in elms" v-bind:key="name[0]" :name="name[0]" :delFun="removeElm" :editFun="editForm"/>
  </div>
</template>

<script>
import AppElement from './AppElement.vue';
import AddOrEdit from './AddOrEdit.vue';
  export default {
    name: 'ListElements',
    data(){
      return {
        elms: new Map(),
        isEdit: false,
        elmLabels: new Set(),
        name: '',
        desc: ''
      }
    },
    components: {
    AddOrEdit,
    AppElement
},
    methods: {
      setIsEdit(vl){
        this.isEdit = vl
      },
      addElm(name,desc,elmLabels){
        console.log("BBBB")
        if(name == '')
          return
        if(this.elms.has(name))
          return
        this.elms.set(name,{desc: desc, elmLabels: elmLabels})
      },
      editElm(name,desc, elmLabels){
        this.elms.set(name,{desc: desc, elmLabels: elmLabels})
        this.isEdit = false
      },
      removeElm(name){
        console.log("AAAAAA");
        this.elms.delete(name);
      },
      editForm(name){
        if(!this.elms.has(name))
          return
          this.name = name
          var obj = this.elms.get(name)
          this.desc = obj.desc
          this.elmLabels = obj.elmLabels
          this.isEdit = true
        }
    },
    props: {
      title: String
    }
  }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
  