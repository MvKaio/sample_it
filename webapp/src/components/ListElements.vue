<template>
  <div class="ListElements">
    <h2>{{title}}</h2>
    <AddOrEdit :hasLabels="true" :isEdit="false" :addFunction="addElm" :editFunction="editElm" ></AddOrEdit>
    <AppElement v-for="elm in elms" v-bind:key=elm.id :id=elm.id :name=elm.name />
  </div>
</template>

<script>
import AppElement from './AppElement.vue';
import AddOrEdit from './AddOrEdit.vue';
  export default {
    name: 'ListElements',
    data(){
      return {
        elms: [],
      }
    },
    components: {
    AddOrEdit,
    AppElement
},
    methods: {
      addElm(name,desc,lbs){
        if(name == '')
          return
        for(var i =0; i < this.elms.length; i++){
          if(this.elms[i].name == name)
            return}
        this.elms.push({name: name, desc: desc, lbs: lbs})
      },
      editElm(name,desc, lbs,x){
        this.elms[x].name = name
        this.elms[x].desc = desc
        this.elms[x].lbs = lbs
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
  