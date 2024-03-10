<template>
  <div class="hello">
    <div class="title">
        <div class="intro">
            <h1 class="tth1">Glowingstone124's blog</h1>
            <h3>{{ msg }}</h3>
        </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Title',
  props: {
    msg: String
  }
}
</script>

<style scoped>
@import "@/css/main.css";
</style>
