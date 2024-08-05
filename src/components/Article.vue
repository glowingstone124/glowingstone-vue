<template>
  <div class="body">
    <div class="md" v-html="output"></div>
  </div>
</template>

<script>
import { ref, onMounted, watch, nextTick } from 'vue';
import { useRoute } from 'vue-router';
import Footer from '@/components/Footer.vue';
import { marked } from 'marked';
import hljs from 'highlight.js';
import 'highlight.js/styles/monokai-sublime.css';
import kotlin from 'highlight.js/lib/languages/kotlin';

// Then register the languages you need

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

    onMounted(async () => {
      articleId.value = route.params.articleId;
      await loadMarkdownContent();
      highlightCodeBlocks();
    });

    async function loadMarkdownContent() {
      hljs.registerLanguage('kotlin', kotlin);
      const fm = require('front-matter');
      try {
        const file = await import(`@root/public/articles/${articleId.value}`);
        output.value = marked.parse(file.default);
        // 确保在内容更新后执行高亮处理
        nextTick(() => highlightCodeBlocks());
      } catch (error) {
        console.error('Error loading Markdown content:', error);
      }
    }

    function highlightCodeBlocks() {
      document.querySelectorAll('pre code').forEach((block) => {
        hljs.highlightElement(block);
      });
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
}
.md:deep(li){
  color: white;
}

.md:deep(code) {
  display: inline-block;
  background-color: #323b46;
  padding: 14px;
  font-family: Jetbrains;
  border: 1px solid #a2aed7;
  border-radius: 8px;
}
.md:deep(span){
  font-family: Jetbrains;
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
