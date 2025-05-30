<template>
	<div class="body">
		<div class="pre">
			<h1 class="articleTitle">{{ articleTitle }}</h1>
			<div class="notice">
				<p>感谢您阅读此文章！本人水平有限，写作时难免出现错误和纰漏。如果您在阅读过程中有任何想法或建议，欢迎前往Github提交issue。</p>
				<a class="hyper-link" href="https://github.com/glowingstone124/glowingstone-vue/issues/new">Github</a>
			</div>
			<span class="ai-generated" v-if="ai_gen">此篇文章已被标注使用AI生成。</span>
		</div>
		<div class="md" v-html="output"></div>
	</div>
	<VisitCounter/>
</template>


<script>
import {ref, onMounted, watch, nextTick} from 'vue';
import {useRoute} from 'vue-router';
import {marked} from 'marked';
import hljs from 'highlight.js';
import matter from 'gray-matter'
import 'highlight.js/styles/monokai-sublime.css';
import 'katex/dist/katex.min.css';
import katex from 'katex';
import VisitCounter from "@/components/VisitCounter.vue";


export default {
	components: {VisitCounter},
	setup() {
		const route = useRoute();
		const articleId = ref(null);
		const output = ref('');
		const ai_gen = ref(false)
		const articleTitle = ref('')
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
			try {
				const file = await import(`@root/public/articles/${articleId.value}`);
				let markdownContent = matter(file.default).content;

				output.value = marked.parse(markdownContent);
				articleTitle.value = matter(file.default).data.title
				ai_gen.value = matter(file.default).data.ai_generated

				output.value = output.value.replace(/\$\$(.+?)\$\$/g, (_, tex) => {
					return katex.renderToString(tex, {throwOnError: false});
				});
				output.value = output.value.replace(/\$(.+?)\$/g, (_, tex) => {
					return katex.renderToString(tex, {throwOnError: false});
				});

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
			articleTitle,
			ai_gen,
		};
	},
};
</script>

<style scoped>
@import "@/css/main.css";
@import url('https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@100..900&family=Noto+Serif+SC:wght@200..900&display=swap');
.body {
	padding: 8vh 4vw;
	box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
	margin: 20vh auto;
	background-color: #323234;
	max-width: 750px;

}

.md:deep(img) {
	max-width: 80%;
	border-radius: 10px;
}

.md:deep(*) {
	font-family: "SimSun", serif;
}

.md:deep(blockquote) {
	padding-top: 1px;
	padding-bottom: 1px;
	border-left: 2px solid #6cc291;
	background: #3f6c51;
}

.md:deep(li) {
	color: #ececec;
	font-size: 1em;
}

.md:deep(code) {
	display: inline-block;
	background-color: #242428;
	padding: 1px;
	margin: 1px;
	font-family: 'Fira Code', serif;
	border: 1px solid #48485e;
	max-width: 100%;
	overflow-y: auto;
	border-radius: 2px;
}

.md:deep(span) {
	font-family: 'Fira Code';
}

.md:deep(a) {
	text-decoration: none;
	font-weight: normal;
	transition: all 0.2s;
	border-bottom: 1px solid #323234;
	color: #bee5b4;
}

.md:deep(a:hover) {
	transition: all 0.2s;
	border-bottom: 1px solid #d6d6e8;
}

.md:deep(h1) {
	font-weight: 900;
	font-size: 50px;
}

.md:deep(h2) {
	font-weight: 300;
	font-size: 40px;
}

.md:deep(h3) {
	font-weight: 300;
	font-size: 25px;
	color: white;
}

.md:deep(hr) {
	display: none;
}

.md:deep(.katex) {
	font-family: 'KaTeX_Main', 'Times New Roman', serif;
	font-size: 1.1em;
}

.md:deep(ul) {
	display: block;
}

.md:deep(p) {
	padding:10px;
	color: #e1e0e0;
	font-size: 1em;
	font-weight: 400;
}

.articleTitle {
	font-weight: 100;
	margin-bottom: 8px;
	font-size: 3.5rem;
	font-family: 'Inter', "Microsoft Sans Serif", serif;
}

.ai-generated {
	border-left: 2px solid #6cc291;
	background: #3f6c51;
	color: #ffffff;
	padding: 20px;
	font-size: 1rem;
}

.notice{
	margin: 2rem;
	background: #6d8175;
	padding: 1rem;
	border-radius: 30px;
	.hyper-link{
		border-radius: 30px;
		border: none;
		margin:0.4rem;
		text-decoration: none;
		background: #b8c7b8;
		padding: 0.5rem;
	}
}

.pre{
	margin-bottom: 100px;
}
@media (max-width: 500px) {
	.md:deep(p) {
		font-size: 0.8rem;
	}
	.body {
		margin: 0 auto;
	}
}
</style>
