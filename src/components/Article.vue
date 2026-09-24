<template>
	<div class="reader-page">
		<!-- Top Reading Progress Indicator -->
		<div class="reading-progress-track">
			<div class="reading-progress-bar" :style="{ width: scrollProgress + '%' }"></div>
		</div>

		<div class="reader-container">
			<!-- Terminal Header & Breadcrumbs -->
			<div class="terminal-nav-header font-mono">
				<router-link to="/archive" class="back-link">
					&larr; [返回文章列表]
				</router-link>
				<span class="file-path-tag">
					/articles/{{ articleId }}
				</span>
			</div>

			<!-- Article Hero Box -->
			<header class="article-hero-box">
				<div class="hero-top-meta font-mono" v-if="ai_gen">
					<span class="ai-banner font-mono">
						[AI 生成: 该文章包含AI辅助生成内容]
					</span>
				</div>

				<h1 class="article-title">{{ articleTitle }}</h1>

				<div class="hero-footer-meta font-mono">
					<span class="word-metric">{{ wordCount }} 字</span>
					<span class="reading-time">预计阅读: ~{{ Math.max(1, Math.ceil(wordCount / 400)) }} 分钟</span>
					<a
						class="github-issue-pill"
						href="https://github.com/glowingstone124/glowingstone-vue/issues/new"
						target="_blank"
						rel="noopener"
					>
						[在 GitHub 提交反馈 &rarr;]
					</a>
				</div>
			</header>

			<!-- Markdown Content Body -->
			<main class="markdown-surface">
				<div class="md-content" v-html="output"></div>
			</main>

			<!-- Post Footer Navigation -->
			<footer class="reader-footer font-mono">
				<router-link to="/archive" class="metro-btn">&larr; 返回文章列表</router-link>
				<button class="metro-btn" @click="scrollToTop">回到顶部 &uarr;</button>
			</footer>
		</div>

		<VisitCounter />
	</div>
</template>

<script>
import { ref, onMounted, onBeforeUnmount, watch, nextTick } from 'vue';
import { useRoute } from 'vue-router';
import { marked } from 'marked';
import hljs from 'highlight.js';
import matter from 'gray-matter';
import 'highlight.js/styles/monokai-sublime.css';
import 'katex/dist/katex.min.css';
import katex from 'katex';
import VisitCounter from "@/components/VisitCounter.vue";

export default {
	name: 'Article',
	components: { VisitCounter },
	setup() {
		const route = useRoute();
		const articleId = ref(null);
		const output = ref('');
		const ai_gen = ref(false);
		const articleTitle = ref('');
		const wordCount = ref(0);
		const scrollProgress = ref(0);

		const handleScroll = () => {
			const totalHeight = document.documentElement.scrollHeight - window.innerHeight;
			if (totalHeight > 0) {
				scrollProgress.value = Math.min(100, Math.max(0, (window.scrollY / totalHeight) * 100));
			}
		};

		const scrollToTop = () => {
			window.scrollTo({ top: 0, behavior: 'smooth' });
		};

		watch(
			() => route.params.articleId,
			(newId) => {
				if (newId) {
					articleId.value = newId;
					loadMarkdownContent();
				}
			}
		);

		onMounted(async () => {
			articleId.value = route.params.articleId;
			window.addEventListener('scroll', handleScroll, { passive: true });
			await loadMarkdownContent();
		});

		onBeforeUnmount(() => {
			window.removeEventListener('scroll', handleScroll);
		});

		async function loadMarkdownContent() {
			try {
				const file = await import(`@root/public/articles/${articleId.value}`);
				const parsed = matter(file.default);
				const rawMarkdown = parsed.content;

				articleTitle.value = parsed.data.title || articleId.value;
				ai_gen.value = parsed.data.ai_generated || false;
				wordCount.value = rawMarkdown.length;

				// Parse markdown
				let html = marked.parse(rawMarkdown);

				// Replace KaTeX math block expressions $$...$$ and inline $...$
				html = html.replace(/\$\$([\s\S]+?)\$\$/g, (_, tex) => {
					try {
						return `<div class="katex-display-box">${katex.renderToString(tex, { displayMode: true, throwOnError: false })}</div>`;
					} catch (e) {
						return `$$${tex}$$`;
					}
				});

				html = html.replace(/\$([^\$\n]+?)\$/g, (_, tex) => {
					try {
						return katex.renderToString(tex, { displayMode: false, throwOnError: false });
					} catch (e) {
						return `$${tex}$`;
					}
				});

				output.value = html;

				nextTick(() => {
					highlightCodeBlocks();
				});
			} catch (error) {
				console.error('Error loading Markdown content:', error);
				output.value = `
					<div class="load-error font-mono">
						<h3>[FILE_NOT_FOUND]</h3>
						<p>无法定位或加载目标文章资源: ${articleId.value}</p>
					</div>
				`;
			}
		}

		function highlightCodeBlocks() {
			document.querySelectorAll('.md-content pre code').forEach((block) => {
				hljs.highlightElement(block);
			});
		}

		return {
			articleId,
			output,
			articleTitle,
			ai_gen,
			wordCount,
			scrollProgress,
			scrollToTop,
		};
	},
};
</script>

<style scoped>
.reader-page {
	width: 100%;
	min-height: 100vh;
	padding-bottom: 120px;
}

/* Scroll Progress Bar */
.reading-progress-track {
	position: fixed;
	top: 64px;
	left: 0;
	width: 100%;
	height: 2px;
	background: #262626;
	z-index: 1001;
}

.reading-progress-bar {
	height: 100%;
	background: #ffffff;
	transition: width 0.08s ease-out;
}

.reader-container {
	max-width: 1140px;
	margin: 20px auto 80px;
	padding: 0 40px;
	box-sizing: border-box;
}

/* Terminal Nav Header */
.terminal-nav-header {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 16px 28px;
	background: #121212;
	border: 1px solid #262626;
	font-size: 0.85rem;
}

.back-link {
	color: #ffffff;
	text-decoration: none;
}

.back-link:hover {
	text-decoration: underline;
}

.file-path-tag {
	color: #737373;
	overflow: hidden;
	text-overflow: ellipsis;
	white-space: nowrap;
	max-width: 50%;
}

/* Article Hero Box */
.article-hero-box {
	background: #121212;
	border: 1px solid #262626;
	border-top: none;
	padding: 56px 64px 44px;
	margin-bottom: 44px;
}

.hero-top-meta {
	margin-bottom: 20px;
}

.ai-banner {
	color: #a3a3a3;
	border: 1px solid #404040;
	padding: 6px 14px;
	font-size: 0.82rem;
}

.article-title {
	font-size: 2.6rem;
	font-weight: 600;
	color: #ffffff;
	line-height: 1.35;
	margin: 0 0 28px;
	letter-spacing: -0.5px;
}

.hero-footer-meta {
	display: flex;
	align-items: center;
	gap: 24px;
	font-size: 0.85rem;
	color: #a3a3a3;
	padding-top: 22px;
	border-top: 1px solid #262626;
	flex-wrap: wrap;
}

.github-issue-pill {
	color: #ffffff;
	text-decoration: none;
	margin-left: auto;
}

.github-issue-pill:hover {
	text-decoration: underline;
}

/* Markdown Surface */
.markdown-surface {
	background: #121212;
	border: 1px solid #262626;
	padding: 72px 80px;
}

.md-content {
	color: #d4d4d4;
	font-size: 1.12rem;
	line-height: 2.0;
}

/* Markdown Inner Styling */
.md-content:deep(h1) {
	font-size: 2.1rem;
	color: #ffffff;
	border-bottom: 1px solid #262626;
	padding-bottom: 14px;
	margin: 56px 0 24px;
	font-weight: 500;
}

.md-content:deep(h2) {
	font-size: 1.65rem;
	color: #ffffff;
	margin: 44px 0 20px;
	font-weight: 500;
}

.md-content:deep(h3) {
	font-size: 1.35rem;
	color: #e5e5e5;
	margin: 36px 0 16px;
	font-weight: 500;
}

.md-content:deep(p) {
	margin-bottom: 28px;
	color: #d4d4d4;
	padding: 0;
}

.md-content:deep(a) {
	color: #ffffff;
	text-decoration: underline;
	transition: all 0.15s ease;
}

.md-content:deep(img) {
	max-width: 100%;
	height: auto;
	border: 1px solid #333333;
	margin: 32px auto;
	display: block;
}

.md-content:deep(blockquote) {
	margin: 32px 0;
	padding: 20px 28px;
	background: #171717;
	border-left: 4px solid #ffffff;
	color: #e5e5e5;
}

.md-content:deep(blockquote p) {
	margin: 0;
	color: #a3a3a3;
}

.md-content:deep(pre) {
	background: #000000;
	border: 1px solid #333333;
	padding: 24px 28px;
	overflow-x: auto;
	margin: 32px 0;
}

.md-content:deep(code) {
	font-family: var(--font-mono);
	font-size: 0.9em;
}

.md-content:deep(:not(pre) > code) {
	background: #1a1a1a;
	color: #ffffff;
	border: 1px solid #333333;
	padding: 2px 8px;
}

.md-content:deep(ul),
.md-content:deep(ol) {
	padding-left: 32px;
	margin-bottom: 28px;
}

.md-content:deep(li) {
	margin-bottom: 12px;
	color: #d4d4d4;
}

.md-content:deep(table) {
	width: 100%;
	border-collapse: collapse;
	margin: 36px 0;
	background: #0a0a0a;
	border: 1px solid #262626;
}

.md-content:deep(th) {
	background: #171717;
	color: #ffffff;
	font-weight: 500;
	padding: 14px 18px;
	border-bottom: 1px solid #333333;
	text-align: left;
}

.md-content:deep(td) {
	padding: 14px 18px;
	border-bottom: 1px solid #262626;
	color: #d4d4d4;
}

.md-content:deep(hr) {
	border: none;
	height: 1px;
	background: #262626;
	margin: 48px 0;
}

/* KaTeX Formula Box */
:deep(.katex-display-box) {
	padding: 24px 28px;
	margin: 32px 0;
	background: #0a0a0a;
	border: 1px solid #262626;
	overflow-x: auto;
}

/* Reader Footer */
.reader-footer {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 32px 0;
	margin-top: 40px;
	border-top: 1px solid #262626;
}

@media screen and (max-width: 768px) {
	.markdown-surface {
		padding: 36px 24px;
	}
	.article-hero-box {
		padding: 32px 24px;
	}
	.article-title {
		font-size: 2rem;
	}
	.reader-container {
		padding: 0 20px;
	}
}
</style>
