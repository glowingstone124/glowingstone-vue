<template>
  <div class="body contents">
    <div class="md" v-html="output"></div>
  </div>
</template>

<script>
import { ref, onMounted, watch } from 'vue';
import { useRoute } from 'vue-router';
import Footer from '@/components/Footer.vue';
import { marked } from 'marked'

export default {
  components: {
    Footer,
  },
  setup() {
    const route = useRoute();
    const articleId = ref(null);
    const output = ref('');

    watch(() => route.params, (newParams) => {
      articleId.value = newParams.articleId;
      loadMarkdownContent();
    });

    onMounted(() => {
      articleId.value = route.params.articleId;
      loadMarkdownContent();
    });

    async function loadMarkdownContent() {
      const fm = require('front-matter');
      try {
        const file = await import("@root/public/articles/" + articleId.value);
       //console.log(file.default)
        output.value = marked.parse(file.default);
        return output.value
      } catch (error) {
      }
    }

    return {
      articleId,
      output,
    };
  },
};
</script>

<style>
@import "@/css/main.css";
.body {
  padding-top: 5vh;
  padding-left: 10vw;
  padding-right: 10vw;
  padding-bottom: 5vh;
}

* {
  color: white;
}
code{
  background-color: #323b46;
  padding: 2px;
  font-family: Jetbrains;
}
a{
  color: white;
}
h1{
  text-decoration: none;
}
</style>
