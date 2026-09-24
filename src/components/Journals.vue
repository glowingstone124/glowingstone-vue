<template>
	<section class="metro-journals-section">
		<!-- Section Header -->
		<div class="section-header">
			<h2 class="section-title">小记 <span class="sub">Journals</span></h2>
			<span class="section-meta font-mono">共 {{ journals.length }} 篇便签 // GLANCE &amp; GO</span>
		</div>

		<!-- Memo Board Grid -->
		<div class="journals-memo-grid">
			<article
				v-for="journal in journals"
				:key="journal.id"
				class="memo-tile"
				:class="[getSpanClass(journal), getPaperTheme(journal)]"
			>
				<!-- Sticky Note Tape (顶部固定胶带) -->
				<div class="memo-tape"></div>

				<!-- Paper Fold (便签折角) -->
				<div class="memo-fold"></div>

				<!-- Tear-off perforation (便签撕纸虚线) -->
				<div class="memo-tear-edge"></div>

				<!-- Memo Header Stamp (便签纸顶部信息戳) -->
				<div class="memo-stamp font-mono">
					<span class="memo-date">{{ journal.time }}</span>
					<span class="memo-tag">NOTE // #{{ String(journal.id).padStart(2, '0') }}</span>
				</div>

				<!-- Memo Title -->
				<h3 class="memo-title font-mono">{{ journal.title }}</h3>

				<!-- Direct Memo Content (内容直接完整呈现，恢复 glance and go 语义) -->
				<div class="memo-content" v-html="journal.content"></div>
			</article>
		</div>
	</section>
</template>

<script>
import { marked } from 'marked';
import fm from 'front-matter';

export default {
	name: 'Journals',
	data() {
		return {
			journals: [],
		};
	},
	created() {
		this.loadJournals();
	},
	methods: {
		loadJournals() {
			try {
				const context = require.context('@root/public/journals', true, /\.md$/);
				const journalFiles = context.keys();
				let count = 0;
				this.journals = journalFiles.map((key) => {
					const fileContent = context(key);
					const data = fm(fileContent);
					count += 1;
					return {
						title: data.attributes.title || '无标题便签',
						time: data.attributes.time || '',
						content: marked.parse(data.body),
						length: data.body ? data.body.length : 0,
						id: count,
					};
				});
			} catch (err) {
				console.error('Failed to load journals:', err);
			}
		},
		getSpanClass(journal) {
			// 长于 70 字的便签占双列，短便签占单列，呈现便签纸错落排布的看板效果
			return journal.length > 70 ? 'span-2' : 'span-1';
		},
		getPaperTheme(journal) {
			// 黑白灰阶不同质感的便签纸主题
			const themes = ['paper-graphite', 'paper-slate', 'paper-carbon'];
			return themes[(journal.id - 1) % themes.length];
		},
	},
};
</script>

<style scoped>
.metro-journals-section {
	max-width: var(--max-content-width);
	margin: 0 auto 120px;
	padding: 0 40px;
	box-sizing: border-box;
}

/* Memo Board Grid with dense auto packing */
.journals-memo-grid {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
	grid-auto-flow: dense;
	gap: 32px;
	width: 100%;
}

/* Base Memo Note Styling (便签纸基底) */
.memo-tile {
	position: relative;
	padding: 38px 36px 34px;
	box-sizing: border-box;
	border: 1px solid #262626;
	display: flex;
	flex-direction: column;
	transition: transform 0.2s ease, border-color 0.2s ease;
	/* 横线纸格纹质感 (Notebook Ruling) */
	background-image: repeating-linear-gradient(
		to bottom,
		transparent,
		transparent 31px,
		rgba(255, 255, 255, 0.035) 31px,
		rgba(255, 255, 255, 0.035) 32px
	);
}

.memo-tile:hover {
	transform: translateY(-5px);
	border-color: #ffffff;
}

/* Dynamic Bento Grid Spans */
.memo-tile.span-2 {
	grid-column: span 2;
}

.memo-tile.span-1 {
	grid-column: span 1;
}

/* Paper Theme Variations (纯正黑白灰阶便签纸质感) */
.paper-graphite {
	background-color: #141414;
	border-color: #2b2b2b;
}

.paper-slate {
	background-color: #181818;
	border-color: #333333;
}

.paper-carbon {
	background-color: #111111;
	border-color: #262626;
}

/* Sticky Note Tape (顶部固定和纸纹理胶带) */
.memo-tape {
	position: absolute;
	top: -9px;
	left: 50%;
	transform: translateX(-50%);
	width: 72px;
	height: 18px;
	background: repeating-linear-gradient(
		-45deg,
		rgba(255, 255, 255, 0.12),
		rgba(255, 255, 255, 0.12) 3px,
		rgba(255, 255, 255, 0.22) 3px,
		rgba(255, 255, 255, 0.22) 6px
	);
	border: 1px solid rgba(255, 255, 255, 0.35);
	backdrop-filter: blur(2px);
	z-index: 2;
	pointer-events: none;
	transition: transform 0.2s ease, border-color 0.2s ease;
}

.memo-tile:hover .memo-tape {
	transform: translateX(-50%) scaleX(1.06);
	border-color: #ffffff;
}

/* Paper Fold (便签纸右下折角加花) */
.memo-fold {
	position: absolute;
	bottom: 0;
	right: 0;
	width: 0;
	height: 0;
	border-style: solid;
	border-width: 0 0 14px 14px;
	border-color: transparent transparent #0a0a0a transparent;
	transition: border-width 0.2s ease, border-color 0.2s ease;
	pointer-events: none;
	z-index: 2;
}

.memo-tile:hover .memo-fold {
	border-width: 0 0 20px 20px;
	border-color: transparent transparent #ffffff transparent;
}

/* Tear-off Perforation (便签顶部撕痕虚线) */
.memo-tear-edge {
	width: 100%;
	border-top: 1px dashed rgba(255, 255, 255, 0.18);
	margin-bottom: 16px;
}

/* Memo Stamp Header (戳记/编号) */
.memo-stamp {
	display: flex;
	justify-content: space-between;
	align-items: center;
	font-size: 0.82rem;
	color: #737373;
	margin-bottom: 14px;
	letter-spacing: 0.5px;
}

.memo-date {
	color: #888888;
}

.memo-tag {
	color: #a3a3a3;
	border: 1px solid #383838;
	padding: 1px 6px;
	font-size: 0.76rem;
	letter-spacing: 1px;
	background: rgba(0, 0, 0, 0.4);
}

/* Memo Title */
.memo-title {
	margin: 0 0 16px;
	font-size: 1.25rem;
	font-weight: 600;
	color: #ffffff;
	line-height: 1.4;
	letter-spacing: -0.2px;
}

/* Direct Content Typography (内容直接展现，易读舒适) */
.memo-content {
	font-size: 0.96rem;
	line-height: 1.85;
	color: #d4d4d4;
	word-break: break-word;
	flex-grow: 1;
}

.memo-content:deep(p) {
	margin: 0 0 12px;
	padding: 0;
}

.memo-content:deep(p:last-child) {
	margin-bottom: 0;
}

.memo-content:deep(code) {
	background: #000000;
	border: 1px solid #383838;
	padding: 2px 6px;
	font-family: var(--font-mono);
	font-size: 0.85em;
	color: #ffffff;
}

.memo-content:deep(blockquote) {
	margin: 12px 0;
	padding-left: 14px;
	border-left: 2px solid #555555;
	color: #a3a3a3;
	font-style: italic;
}

.memo-content:deep(a) {
	color: #ffffff;
	text-decoration: underline;
}

@media screen and (max-width: 768px) {
	.journals-memo-grid {
		grid-template-columns: 1fr;
		gap: 24px;
	}

	.memo-tile.span-2 {
		grid-column: span 1;
	}

	.metro-journals-section {
		padding: 0 20px;
	}

	.memo-tile {
		padding: 30px 24px 26px;
	}
}
</style>
