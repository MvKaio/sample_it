<template>
  <div class="ListElements">
    <h2>{{title}}</h2>
    <AddOrEdit></AddOrEdit>
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
        elms: [{name: "AAAAA", id: 1},{name: "BBB", id: 2}]
      }
    },
    components: {
    AddOrEdit,
    AppElement
},
    props: {
      title: String
    }
  }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
  