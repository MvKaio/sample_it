<template>
    <div class="ShowLabel">
      <span>{{name}}</span> <span v-show="deleteFunction != null" @click="deleteFunction(name)">X</span>
    </div>
  </template>
  
  <script>
  export default {
    name: 'MainHeader',
    props: {
        name: String,
        deleteFunction: {
            type: Function,
            default: null
        }
    }
  }
  </script>
  
  <!-- Add "scoped" attribute to limit CSS to this component only -->
  <style scoped>
      .ShowLabel {
          border-style: solid;
          display: inline-block;
      }
      span {
        display: inline-block;
        padding-right: 10px;
      }
  </style>
  