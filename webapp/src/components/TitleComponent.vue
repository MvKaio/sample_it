<script setup>

defineProps({
    title: String
})

</script>
    
<template>
    <h1 class=" drop-shadow-lg my-7 text-neutral-50 mx-auto text-6xl">{{title}}</h1>
</template>

<style scoped>
h1 {
    text-shadow: black 5px 4px;
    user-select: none;
}
</style>