<template>
	<div class="mobile-articles">
		<div class="article-item" v-for="article in articles" :key="article.id">
			<ArticleCard :article="article"/>
		</div>
		<VisitCounter/>
	</div>
</template>

<script>
import {ref} from 'vue';
import path from 'path';
import fm from "front-matter";
import ArticleCard from "@/components/Mobile/ArticleCard.vue";
import VisitCounter from "@/components/VisitCounter.vue";

export default {
	name: 'MobileArticleList',
	components: {VisitCounter, ArticleCard},
	setup() {
		const articles = ref([]);

		(async () => {
			const files = require.context("@root/public/articles/", true, /\.md$/);
			const filelist = files.keys();

			for (const key of filelist) {
				const data = fm(files(key));
				const fileName = path.basename(key);

				let imagePath = '/default.jpg';
				const picture = data.attributes.picture;
				const ai = data.attributes.ai_generated;
				let category = '';
				switch (data.attributes.category) {
					case 0:
						category = '技术';
						break;
					case 1:
						category = '闲聊';
						break;
					case 2:
						category = '时事';
						break;
					default:
						category = '杂记';
				}
				if (picture && picture !== "none") {
					imagePath = picture;
				}

				articles.value.push({
					id: fileName,
					title: data.attributes.title || fileName,
					abstract: data.attributes.abstract || '',
					imagePath: imagePath,
					category: category,
					ai_generated: ai
				});
			}
		})();

		return {articles};
	}
};
</script>

<style scoped>
.mobile-articles {
	display: flex;
	flex-direction: column;
	gap: 12px;
	margin-top: 16px;
}
</style>
