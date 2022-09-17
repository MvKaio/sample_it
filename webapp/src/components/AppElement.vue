<template>
    <div class="AppElement">
        <span class="name">{{name}}</span> <span class="edit">Edit</span> <span class="delete">Delete</span>
    </div>
</template>
  
<script>
    export default {
    name: 'AppElement',
    props: {
        name: String,
        id: Number
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
  