<template>
	<div class="container">
		<Title :msg="msg"/>
		<h1 class="introduction">小记</h1>
		<Journals />
		<Pinned/>
		<Footer/>
	</div>
	<VisitCounter/>
</template>

<script>
import VisitCounter from "@/components/VisitCounter.vue";
import Title from './Title.vue'
import ArticleList from './ArticleList.vue'
import NavBtn from './NavBtn.vue';
import Footer from './Footer.vue';
import Pinned from './Pinned.vue';
import Journals from './Journals.vue'
export default {
	name: 'App',
	components: {
		VisitCounter,
		Title,
		Journals,
		Pinned,
		Footer
	},
	data() {
		return {
			articleList: [],
			visitCount: 114514,
			msg: "I got electric light," +
				"And I got second sight.\n" +
				"Got amazing powers of observation.\n" +
				"And that is how I know,\n" +
				"When I try to get through," +
				"On the telephone to you,\n" +
				"There'll be nobody home.\n" + "- Pink Floyd 《The Wall》"
		}
	},
	async mounted() {
		if (window.innerWidth < 800 || /mobile/i.test(navigator.userAgent)) {
			this.$router.push('/mobile');
		}
		const currentDate = new Date();
		const Birthday = new Date("2025-04-28");
		if (currentDate === Birthday) {
			this.msg = "生日快乐,Never Grow Old.";
			document.title = "Glowstone 萤石 | 祝我生日快乐"
		}
	},
	methods: {}

}
</script>

<style>
@import "@/css/main.css";

p {
	padding: 10px;
	color: white;
	margin: auto;
}

.counter {
	text-align: center;
}

</style>
