<template>
	<div class="apple-journals-section">
		<!-- Section Header -->
		<div class="journals-header">
			<span class="apple-badge">QUICK READS &amp; NOTES</span>
			<h2 class="journals-heading">小记与便签</h2>
			<p class="journals-subheading">碎片化的技术思考、阅读笔记与即刻闪念（共 {{ journals.length }} 则）。</p>
		</div>

		<!-- Apple Notes Grid -->
		<div class="journals-grid">
			<article
				v-for="journal in journals"
				:key="journal.id"
				class="apple-note-card"
			>
				<div class="note-top-bar">
					<span class="note-pill">#{{ String(journal.id).padStart(2, '0') }}</span>
					<span class="note-time">{{ journal.time }}</span>
				</div>

				<h3 class="note-title">{{ journal.title }}</h3>

				<div class="note-content" v-html="journal.content"></div>
			</article>
		</div>
	</div>
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
						title: data.attributes.title || '便签',
						time: data.attributes.time || '2026年',
						content: marked.parse(data.body),
						length: data.body ? data.body.length : 0,
						id: count,
					};
				});
			} catch (err) {
				console.error('Failed to load journals:', err);
			}
		},
	},
};
</script>

<style scoped>
.apple-journals-section {
	width: 100%;
	padding: 24px 0 0;
}

.journals-header {
	margin-bottom: 28px;
}

.journals-heading {
	font-size: 32px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 8px 0 10px;
	letter-spacing: -0.02em;
}

.journals-subheading {
	font-size: 16px;
	color: #86868b;
	margin: 0;
}

.journals-grid {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
	gap: 24px;
}

.apple-note-card {
	background: #ffffff;
	border: 1px solid rgba(0, 0, 0, 0.08);
	border-radius: var(--apple-radius-card, 24px);
	padding: 28px 24px;
	box-shadow: 0 4px 18px rgba(0, 0, 0, 0.04);
	display: flex;
	flex-direction: column;
	transition: transform 0.25s ease, box-shadow 0.25s ease;
}

.apple-note-card:hover {
	transform: translateY(-4px);
	box-shadow: 0 12px 28px rgba(0, 0, 0, 0.08);
}

.note-top-bar {
	display: flex;
	justify-content: space-between;
	align-items: center;
	margin-bottom: 14px;
}

.note-pill {
	font-size: 11px;
	font-weight: 700;
	padding: 3px 10px;
	border-radius: 980px;
	background: #f5f5f7;
	color: #1d1d1f;
}

.note-time {
	font-size: 12px;
	color: #86868b;
}

.note-title {
	font-size: 18px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 0 0 12px;
	letter-spacing: -0.01em;
}

:deep(.note-content) {
	font-size: 14px;
	line-height: 1.65;
	color: #515154;
	flex: 1;
}

:deep(.note-content p) {
	margin: 0 0 10px;
}

:deep(.note-content p:last-child) {
	margin-bottom: 0;
}
</style>
