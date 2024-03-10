<template>
    <div class="body">
        <div class="md" v-html="output">
    </div>
    </div>
    <Footer Footermsg="design & code by glowingstone124"/>
</template>
  <script>
  import { marked } from 'marked';
  import { ref, onMounted } from 'vue';
  import Footer from '@/components/Footer.vue'
  import Title from '@/components/Title.vue'
  export default {
    components: {
    Footer,
    Title,
  },
    setup() {
      const input = ref('# Hello \n ## This');
      const output = ref('');
      onMounted(() => {
        output.value = marked.parse(input.value);
      });
  
      return {
        output,
      };
    },
  };
  </script>
  
  <style>
  @import "@/css/main.css";
  .body{
    padding-top: 5vh;
    padding-left: 10vw;
    padding-right: 10vw;
    padding-bottom: 5vh;
  }
  *{
    color:white
  }

  </style>
  