<template>
    <MainHeader/>
    <ListElements title="Items" />
  </template>

<script>
import MainHeader from './components/MainHeader.vue'
import ListElements from './components/ListElements.vue';

export default {
  data(){
    return {}
  },
  name: 'App',
  components: {
    MainHeader,
    ListElements
}
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
