<template>
	<div class="home-page-container">
		<DitherBackground />
		<div class="home-content-layer">
			<Title :msg="msg"/>
			<AsciiMarquee />
			<Journals />
			<Pinned />
			<VisitCounter />
		</div>
	</div>
</template>

<script>
import VisitCounter from "@/components/VisitCounter.vue";
import Title from './Title.vue';
import Journals from './Journals.vue';
import Pinned from './Pinned.vue';
import AsciiMarquee from './AsciiMarquee.vue';
import DitherBackground from './DitherBackground.vue';

export default {
	name: 'HomePage',
	components: {
		DitherBackground,
		VisitCounter,
		Title,
		AsciiMarquee,
		Journals,
		Pinned,
	},
	data() {
		return {
			msg: "I got electric light, and I got second sight.\n" +
				"Got amazing powers of observation.\n" +
				"And that is how I know,\n" +
				"When I try to get through on the telephone to you,\n" +
				"There'll be nobody home.\n" +
				"— Pink Floyd 《The Wall》",
		};
	},
	mounted() {
		const now = new Date();
		// Birthday check (April 28)
		if (now.getMonth() === 3 && now.getDate() === 28) {
			this.msg = "生日快乐，Never Grow Old.\n祝我生日快乐。";
			document.title = "Glowstone 萤石 | 祝我生日快乐";
		}
	},
};
</script>

<style scoped>
.home-page-container {
	position: relative;
	width: 100%;
	margin: 0 auto;
	box-sizing: border-box;
}

.home-content-layer {
	position: relative;
	z-index: 2;
	width: 100%;
}
</style>
