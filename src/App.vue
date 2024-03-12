<template>
  <Nav/>
  <NavBtn/>
  <div>
  <router-view></router-view>
  </div>
  <Footer Footermsg="design & code by glowingstone124"/>
</template>
<script>
import NavBtn from './components/NavBtn.vue';
import Nav from './components/Nav.vue';
import Footer from './components/Footer.vue';
export default {
  name: 'App',
  components: {
    NavBtn,
    Nav,
    Footer
  },
}
</script>
<style>
@import "@/css/main.css";
</style>
