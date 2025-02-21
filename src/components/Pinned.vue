<template>
	<div class="content">
		<h1 class="introduction">今日推荐</h1>
		<div class="article" v-for="article in articles" :key="article.id" @click="redirect(article.id)">
			<ArticleCard :article="article" />
		</div>
	</div>
</template>

<script>
import fm from 'front-matter';
import {ref} from "vue";
import ArticleCard from "@/components/ArticleCard_Next.vue";

export default {
	name: 'Pinned',
	components: {ArticleCard},
	setup() {
		const pins = ["Travelogue_of_Changning.md","simple_intro_of_jmm.md", "blogging-evolution.md"];
		const articles = ref([]);

		const redirect = (articleId) => {
			window.location.href = `/article/${articleId}`;
		};

		(async () => {
			try {
				for (const key of pins) {
					const file = await import(`@root/public/articles/${key}`);
					const data = fm(file.default);

					let imagePath = '/default.jpg';
					const picture = data.attributes.picture;

					if (picture && picture !== "none") {
						imagePath = picture;
					}
					let category = ''
					switch (data.attributes.category) {
						case 0:
							category = '技术'
							break;
						case 1:
							category = '闲聊'
							break;
						case 2:
							category = '时事'
							break;
						default:
							category = '未知。。。'
					}
					const article = {
						id: key,
						title: data.attributes.title,
						abstract: data.attributes.abstract,
						imagePath: imagePath,
						category: category,
					};

					articles.value.push(article);
				}
			} catch (error) {
				console.error("Error loading articles:", error);
			}
		})();

		return { articles, redirect };
	}
};

</script>

<style scoped>
</style>
