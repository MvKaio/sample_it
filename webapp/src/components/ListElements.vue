<template>
    <div class="ListElements">
      <h3>{{title}}</h3>
    </div>
  </template>
  
  <script>
  export default {
    name: 'ListElements',
    props: {
      title: String
    }
  }
  </script>
  
  <!-- Add "scoped" attribute to limit CSS to this component only -->
  <style scoped>
  </style>
  