<template>
	<div class="journal-grid">
		<div
			v-for="journal in journals"
			:key="journal.id"
			class="journal-tile"
			:style="getTileStyle(journal)"
		>
			<h3>{{ journal.title }}</h3>
			<small>{{ journal.time }}</small>
			<div class="content" v-html="journal.content"></div>
		</div>
	</div>
</template>

<script>
import { marked } from 'marked';
import fm from 'front-matter';

export default {
	data() {
		return {
			journals: [],
			id: 0,
		};
	},
	created() {
		this.loadJournals();
	},
	methods: {
		loadJournals() {
			const context = require.context('@root/public/journals', true, /\.md$/);
			const journalFiles = context.keys();
			this.journals = journalFiles.map(key => {
				const fileContent = context(key);
				const data = fm(fileContent);
				this.id += 1;
				return {
					title: data.attributes.title,
					time: data.attributes.time,
					content: marked(data.body),
					length: data.body.length,
					id: this.id,
				};
			});
		},
		getTileStyle(journal) {
			const columnSpan = Math.ceil(journal.length / 90);
			return {
				'grid-column': `span ${Math.min(columnSpan, 4)}`,
				'grid-row': 'span 1',
			};
		},
	}
};
</script>

<style scoped>
.journal-grid {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
	gap: 16px;
	max-width: 1200px;
	margin: 80px auto;
	padding: 20px;
}

.journal-tile {
	padding: 15px;
	background-color: #3c69b6;
	transition: transform 0.2s;
}

.journal-tile:hover {
	transform: translateY(-5px);
}

h3 {
	margin: 0;
	font-size: 20px;
	color: #efeeee;
}

small {
	color: #d5cdcd;
}

.content {
	overflow: hidden;
	max-height: 500px;
}
</style>
