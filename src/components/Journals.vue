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
			const colors = ['#4e2f80', '#17467c', '#2d8658', '#a98140'];
			const randomColor = colors[ Math.floor(Math.random() * 10)];

			const columnSpan = Math.min(Math.ceil(journal.length / 60), 3);
			const rowSpan = Math.min(Math.ceil(journal.length / 120), 4);

			return {
				'grid-column': `span ${columnSpan}`,
				'grid-row': `span ${rowSpan}`,
				'background-color': randomColor,
				'transition': 'transform 0.2s, background-color 0.5s',
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
	background-color: #395077;
	transition: transform 0.2s;
}

.journal-tile:hover {
	transform: translateY(-5px);
}

h3 {
	margin: 0;
	font-size: 3rem;
	font-weight: 100;
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
