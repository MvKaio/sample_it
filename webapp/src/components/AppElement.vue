<template>
    <div class="AppElement">
        <span class="name">{{name}}</span> <span class="edit" @click="editFun(name)">Edit</span> <span class="delete" @click="delFun(name)">Delete</span>
    </div>
</template>
  
<script>
    export default {
    name: 'AppElement',
    props: {
        name: String,
        id: Number,
        delFun: Function,
        editFun: Function
    }
  }
</script>
  
  <!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
    .name{
        padding-right: 15%;
    }
    span{
        display: inline-block;
        padding-right: 20px;
    }
</style>
  