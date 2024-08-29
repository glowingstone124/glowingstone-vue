<template>
    <footer>{{ Footermsg }}</footer>

  </template>
  
  <script>
  export default {
    name: 'Footer',
    props: {
      Footermsg: String
    }
  }
  </script>
  
  <style scoped>
  @import "@/css/main.css";
  </style>
  