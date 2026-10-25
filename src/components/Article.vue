<template>
	<div class="newsroom-article-page">
		<!-- Top Reading Progress Indicator -->
		<div class="reading-progress-track">
			<div class="reading-progress-bar" :style="{ width: scrollProgress + '%' }"></div>
		</div>

		<div class="article-reader-container">
			<!-- Top Breadcrumb Navigation -->
			<div class="article-top-nav">
				<router-link to="/archive" class="apple-back-pill">
					<svg width="14" height="14" viewBox="0 0 16 16" fill="none">
						<path d="M10 12.5L5.5 8 10 3.5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
					</svg>
					全部文章
				</router-link>
				<span class="article-id-pill">{{ articleId }}</span>
			</div>

			<!-- Article Header -->
			<header class="article-header">
				<div class="header-meta-row">
					<span class="category-badge">{{ articleCategory }}</span>
					<span class="meta-dot">·</span>
					<span class="article-date">{{ articleDate }}</span>
					<span v-if="ai_gen" class="ai-badge">AI 辅助内容</span>
				</div>

				<h1 class="article-main-title">{{ articleTitle }}</h1>

				<div class="article-stats-bar">
					<div class="stats-left">
						<span class="author-label">作者: <strong>Glowingstone</strong></span>
						<span class="meta-dot">·</span>
						<span class="word-metric">{{ wordCount }} 字</span>
						<span class="meta-dot">·</span>
						<span class="reading-time">约 {{ Math.max(1, Math.ceil(wordCount / 400)) }} 分钟阅读</span>
					</div>

					<a
						class="github-feedback-link"
						href="https://github.com/glowingstone124/glowingstone-vue/issues/new"
						target="_blank"
						rel="noopener"
					>
						<svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
							<path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
						</svg>
						在 GitHub 提交讨论
					</a>
				</div>

				<!-- Cover Image Banner -->
				<div v-if="coverImage && coverImage !== 'none'" class="article-cover-banner">
					<img :src="coverImage" :alt="articleTitle" class="cover-banner-img" />
				</div>
			</header>

			<!-- Markdown Content Body -->
			<main class="article-content-body">
				<div class="apple-markdown" v-html="output"></div>
			</main>

			<!-- Reader Footer Navigation -->
			<footer class="article-reader-footer">
				<router-link to="/archive" class="apple-btn-pill">
					&larr; 返回所有文章
				</router-link>
				<button class="apple-btn-pill primary" @click="scrollToTop">
					回到顶部 &uarr;
				</button>
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
import 'highlight.js/styles/github.css';
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
		const articleCategory = ref('UPDATE');
		const articleDate = ref('October 2026');
		const coverImage = ref(null);
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

		const setupMarkdownRenderer = () => {
			const renderer = new marked.Renderer();

			// Apple-style macOS Code Block with Traffic Light Dots
			renderer.code = function({ text, lang }) {
				const validLang = hljs.getLanguage(lang) ? lang : 'plaintext';
				const highlighted = hljs.highlight(text, { language: validLang }).value;
				return `
					<div class="macos-code-window">
						<div class="macos-window-header">
							<div class="window-controls">
								<span class="control-dot close"></span>
								<span class="control-dot minimize"></span>
								<span class="control-dot zoom"></span>
							</div>
							<span class="window-lang-tag">${validLang}</span>
						</div>
						<pre class="hljs"><code class="language-${validLang}">${highlighted}</code></pre>
					</div>
				`;
			};

			renderer.image = function({ href, title, text }) {
				return `<div class="apple-image-wrapper"><img src="${href}" alt="${text || ''}" title="${title || ''}" class="apple-post-img" /><span class="img-caption">${text || ''}</span></div>`;
			};

			marked.setOptions({
				renderer: renderer,
				gfm: true,
				breaks: true,
			});
		};

		const renderMathFormulas = () => {
			nextTick(() => {
				const container = document.querySelector('.apple-markdown');
				if (!container) return;

				const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT);
				const nodes = [];
				let currentNode;
				while ((currentNode = walker.nextNode())) {
					nodes.push(currentNode);
				}

				for (const node of nodes) {
					if (node.parentElement && (node.parentElement.tagName === 'CODE' || node.parentElement.tagName === 'PRE')) {
						continue;
					}
					const text = node.nodeValue;
					if (!text) continue;

					// Display math $$...$$
					if (text.includes('$$')) {
						const parts = text.split('$$');
						if (parts.length >= 3) {
							const span = document.createElement('span');
							for (let i = 0; i < parts.length; i++) {
								if (i % 2 === 1) {
									try {
										const mathSpan = document.createElement('span');
										katex.render(parts[i], mathSpan, { displayMode: true, throwOnError: false });
										span.appendChild(mathSpan);
									} catch (e) {
										span.appendChild(document.createTextNode('$$' + parts[i] + '$$'));
									}
								} else if (parts[i]) {
									span.appendChild(document.createTextNode(parts[i]));
								}
							}
							if (node.parentNode) {
								node.parentNode.replaceChild(span, node);
							}
							continue;
						}
					}

					// Inline math $...$
					if (text.includes('$')) {
						const regex = /\$([^$\n]+)\$/g;
						if (regex.test(text)) {
							const span = document.createElement('span');
							let lastIdx = 0;
							regex.lastIndex = 0;
							let match;
							while ((match = regex.exec(text)) !== null) {
								if (match.index > lastIdx) {
									span.appendChild(document.createTextNode(text.substring(lastIdx, match.index)));
								}
								try {
									const mathSpan = document.createElement('span');
									katex.render(match[1], mathSpan, { displayMode: false, throwOnError: false });
									span.appendChild(mathSpan);
								} catch (e) {
									span.appendChild(document.createTextNode(match[0]));
								}
								lastIdx = regex.lastIndex;
							}
							if (lastIdx < text.length) {
								span.appendChild(document.createTextNode(text.substring(lastIdx)));
							}
							if (node.parentNode) {
								node.parentNode.replaceChild(span, node);
							}
						}
					}
				}
			});
		};

		const loadMarkdownContent = async () => {
			if (!articleId.value) return;
			try {
				const file = await import(`@root/public/articles/${articleId.value}`);
				const parsed = matter(file.default);

				articleTitle.value = parsed.data.title || articleId.value;
				ai_gen.value = parsed.data.ai_generated || false;
				coverImage.value = (parsed.data.picture && parsed.data.picture !== 'none') ? parsed.data.picture : null;
				articleDate.value = parsed.data.date || 'October 2026';

				switch (parsed.data.category) {
					case 0: articleCategory.value = 'TECH'; break;
					case 1: articleCategory.value = 'STORIES'; break;
					case 2: articleCategory.value = 'PRESS RELEASE'; break;
					case 3: articleCategory.value = 'CREATIVE'; break;
					default: articleCategory.value = 'UPDATE';
				}

				const rawContent = parsed.content;
				wordCount.value = rawContent.replace(/[\s\p{P}]/gu, '').length;

				output.value = marked(rawContent);
				document.title = `${articleTitle.value} · Glowstone Newsroom`;

				renderMathFormulas();
			} catch (err) {
				console.error("Failed to load article:", err);
				output.value = `<div class="error-msg">未能成功加载文章内容: ${err.message}</div>`;
			}
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

		onMounted(() => {
			setupMarkdownRenderer();
			articleId.value = route.params.articleId;
			loadMarkdownContent();
			window.addEventListener('scroll', handleScroll, { passive: true });
		});

		onBeforeUnmount(() => {
			window.removeEventListener('scroll', handleScroll);
		});

		return {
			articleId,
			articleTitle,
			articleCategory,
			articleDate,
			coverImage,
			ai_gen,
			output,
			wordCount,
			scrollProgress,
			scrollToTop,
		};
	},
};
</script>

<style scoped>
.newsroom-article-page {
	width: 100%;
	padding: 24px 0 80px;
}

/* Reading Progress Bar */
.reading-progress-track {
	position: fixed;
	top: 108px;
	left: 0;
	width: 100%;
	height: 3px;
	background: transparent;
	z-index: 999;
}

.reading-progress-bar {
	height: 100%;
	background: #0071e3;
	transition: width 0.1s ease-out;
}

.article-reader-container {
	max-width: 860px;
	margin: 0 auto;
	padding: 0 24px;
	box-sizing: border-box;
}

/* Top Nav */
.article-top-nav {
	display: flex;
	align-items: center;
	justify-content: space-between;
	margin-bottom: 32px;
}

.apple-back-pill {
	display: inline-flex;
	align-items: center;
	gap: 6px;
	font-size: 14px;
	color: #0071e3;
	font-weight: 500;
	padding: 6px 14px;
	border-radius: 980px;
	background: #f5f5f7;
	transition: background-color 0.18s ease;
}

.apple-back-pill:hover {
	background: #e8e8ed;
}

.article-id-pill {
	font-size: 12px;
	color: #86868b;
	padding: 4px 12px;
	background: #f5f5f7;
	border-radius: 980px;
}

/* Header */
.article-header {
	margin-bottom: 48px;
}

.header-meta-row {
	display: flex;
	align-items: center;
	gap: 8px;
	margin-bottom: 16px;
}

.category-badge {
	font-size: 12px;
	font-weight: 700;
	letter-spacing: 0.06em;
	color: #86868b;
	text-transform: uppercase;
}

.article-date {
	font-size: 13px;
	color: #86868b;
}

.meta-dot {
	color: #d2d2d7;
}

.ai-badge {
	font-size: 11px;
	font-weight: 600;
	color: #0071e3;
	background: #e8f2fd;
	padding: 2px 8px;
	border-radius: 980px;
}

.article-main-title {
	font-size: 42px;
	font-weight: 700;
	line-height: 1.15;
	color: #1d1d1f;
	margin: 0 0 24px;
	letter-spacing: -0.025em;
}

.article-stats-bar {
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding-bottom: 24px;
	border-bottom: 1px solid var(--apple-border, #e5e5ea);
	font-size: 13px;
	color: #86868b;
	flex-wrap: wrap;
	gap: 12px;
}

.stats-left {
	display: flex;
	align-items: center;
	gap: 8px;
}

.author-label strong {
	color: #1d1d1f;
}

.github-feedback-link {
	display: inline-flex;
	align-items: center;
	gap: 6px;
	color: #86868b;
	font-size: 13px;
	transition: color 0.15s ease;
}

.github-feedback-link:hover {
	color: #0071e3;
}

/* Cover Banner */
.article-cover-banner {
	margin-top: 32px;
	border-radius: var(--apple-radius-card, 24px);
	overflow: hidden;
	box-shadow: 0 8px 30px rgba(0, 0, 0, 0.08);
}

.cover-banner-img {
	width: 100%;
	max-height: 500px;
	object-fit: cover;
	display: block;
}

/* Content Body */
.article-content-body {
	margin-bottom: 64px;
}

/* Apple Typography for Markdown */
:deep(.apple-markdown) {
	font-size: 17px;
	line-height: 1.8;
	color: #1d1d1f;
}

:deep(.apple-markdown p) {
	margin: 0 0 24px;
	color: #333336;
}

:deep(.apple-markdown h2) {
	font-size: 28px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 48px 0 20px;
	letter-spacing: -0.015em;
}

:deep(.apple-markdown h3) {
	font-size: 22px;
	font-weight: 600;
	color: #1d1d1f;
	margin: 36px 0 16px;
}

:deep(.apple-markdown blockquote) {
	margin: 28px 0;
	padding: 18px 24px;
	background: #f5f5f7;
	border-left: 4px solid #0071e3;
	border-radius: 0 16px 16px 0;
	color: #515154;
	font-style: normal;
}

:deep(.apple-markdown blockquote p) {
	margin: 0;
}

:deep(.apple-markdown ul),
:deep(.apple-markdown ol) {
	margin: 0 0 24px;
	padding-left: 28px;
}

:deep(.apple-markdown li) {
	margin-bottom: 8px;
	color: #333336;
}

:deep(.apple-markdown table) {
	width: 100%;
	border-collapse: collapse;
	margin: 32px 0;
	border-radius: 12px;
	overflow: hidden;
	border: 1px solid #e5e5ea;
}

:deep(.apple-markdown th),
:deep(.apple-markdown td) {
	padding: 12px 18px;
	text-align: left;
	border-bottom: 1px solid #e5e5ea;
	font-size: 15px;
}

:deep(.apple-markdown th) {
	background: #f5f5f7;
	font-weight: 600;
	color: #1d1d1f;
}

:deep(.apple-markdown tr:last-child td) {
	border-bottom: none;
}

/* macOS Terminal Window Code Blocks */
:deep(.macos-code-window) {
	margin: 32px 0;
	background: #1e1e1e;
	border-radius: 14px;
	overflow: hidden;
	box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
}

:deep(.macos-window-header) {
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding: 10px 16px;
	background: #2d2d2d;
	border-bottom: 1px solid #3d3d3d;
}

:deep(.window-controls) {
	display: flex;
	gap: 7px;
}

:deep(.control-dot) {
	width: 11px;
	height: 11px;
	border-radius: 50%;
	display: inline-block;
}

:deep(.control-dot.close) { background: #ff5f56; }
:deep(.control-dot.minimize) { background: #ffbd2e; }
:deep(.control-dot.zoom) { background: #27c93f; }

:deep(.window-lang-tag) {
	font-size: 11px;
	color: #999999;
	text-transform: uppercase;
	font-family: var(--apple-font-mono, monospace);
}

:deep(.macos-code-window pre) {
	margin: 0;
	padding: 18px 20px;
	overflow-x: auto;
	background: #1e1e1e !important;
}

:deep(.macos-code-window code) {
	font-family: var(--apple-font-mono, monospace);
	font-size: 14px;
	line-height: 1.6;
	color: #d4d4d4;
}

/* Inline Code */
:deep(.apple-markdown :not(pre) > code) {
	padding: 3px 6px;
	background: #f5f5f7;
	border: 1px solid #e5e5ea;
	border-radius: 6px;
	font-family: var(--apple-font-mono, monospace);
	font-size: 14px;
	color: #eb5757;
}

/* Images in Markdown */
:deep(.apple-image-wrapper) {
	margin: 32px 0;
	text-align: center;
}

:deep(.apple-post-img) {
	max-width: 100%;
	border-radius: 16px;
	box-shadow: 0 4px 20px rgba(0, 0, 0, 0.06);
	display: inline-block;
}

:deep(.img-caption) {
	display: block;
	margin-top: 10px;
	font-size: 13px;
	color: #86868b;
}

/* Footer Navigation */
.article-reader-footer {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding-top: 32px;
	border-top: 1px solid var(--apple-border, #e5e5ea);
}

@media screen and (max-width: 768px) {
	.article-main-title {
		font-size: 30px;
	}
	:deep(.apple-markdown) {
		font-size: 16px;
	}
}
</style>
