<template>
  <button class="navigatebtn" @click="toggleNavigation">
    <img src="@/assets/nav.svg" alt="Navigation">
  </button>
</template>

<script>
export default {
  name: 'NavBtn',
  data() {
    return {
      navStat: false
    };
  },
  methods: {
    toggleNavigation() {
      const navigate = document.getElementById('navigation');
      if (navigate) {
        if (this.navStat) {
          navigate.classList.remove('display');
        } else {
          navigate.classList.add('display');
        }
        this.navStat = !this.navStat;
      } else {
        console.error("Element with id 'navigation' not found.");
      }
    }
  },
  mounted() {
  }
}
</script>

<style scoped>
@import "@/css/main.css";
</style>
