<template>
  <div class="body">
    <div class="md" v-html="output"></div>
  </div>
</template>

<script>
import { ref, onMounted, watch } from 'vue';
import { useRoute } from 'vue-router';
import Footer from '@/components/Footer.vue';
import { marked } from 'marked';
import hljs from 'highlight.js';
import 'highlight.js/styles/github.css';

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
      document.addEventListener('DOMContentLoaded', (event) => {
        document.querySelectorAll('code').forEach((block) => {
          hljs.highlightBlock(block);
        });
      });
    });

    async function loadMarkdownContent() {
      const fm = require('front-matter');
      try {
        const file = await import("@root/public/articles/" + articleId.value);
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

<style scoped>
@import "@/css/main.css";
.body {
  padding: 5vh 4vw;
  margin: 5vw;
}

.md:deep(*) {
  color: white;
}

.md:deep(code) {
  display: inline-block;
  background-color: #323b46;
  padding: 12px;
  font-family: Jetbrains;
  border: 1px solid #a2aed7;
  border-radius: 8px;
}
.md:deep(a) {
  text-decoration: none;
  font-weight: bold;
  transition: all 0.2s;
}

.md:deep(a:hover) {
  text-decoration: underline; /* 修改为只设置下划线 */
}

.md:deep(h1) {
  font-weight: 300;
  font-size: 50px;
}

.md:deep(h2) {
  font-weight: 300;
  font-size: 30px;
  border-bottom: 1px solid rgb(172, 191, 239);
}

.md:deep(h3) {
  font-weight: 300;
  font-size: 20px;
  color: white;
}

.md:deep(hr) {
  display: none;
}
.md:deep(ul){
  display: block;
}
</style>
