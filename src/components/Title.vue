<template>
	<div class="hello">
		<div class="title">
			<div class="intro">
				<h1 class="tth1"><span style="font-weight: 200;">Glowstone </span><span class="light">萤石</span></h1>
				<h3 v-html="msgWithBr" class="quote"></h3>
			</div>
		</div>
		<div class="myself">
			<div class="intro">
				<h1 class="msh1">关于我</h1>
				<iframe class="github" frameBorder="0" height="231.5px" width="1228px"
						src="https://git-graph.vercel.app/embed/glowingstone124?showColorLegend=true&showWeekdayLabels=true&showMonthLabels=true&showTotalCount=true&blockMargin=5&blockRadius=3&blockSize=17&fontSize=17&weekStart=0&year=2024"></iframe>
				<h3>从小学<span>四年级</span>开始学习 <span>Java</span></h3>
				<h3>拥有<span>六年</span>Java编码经验，正在学习<span>Kotlin, C++, OpenGL & Vulkan</span>，程序的最终目标是以<span>简单，优雅</span>的方式高效处理问题。
				</h3>
				<h3>技术栈：</h3>
				<img src="https://img.shields.io/badge/Java-ED8B00?style=for-the-badge&logo=openjdk&logoColor=white">
				<img src="https://img.shields.io/badge/Kotlin-0095D5?&style=for-the-badge&logo=kotlin&logoColor=white">
				<img src="https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vue.js&logoColor=4FC08D">
				<img src="https://img.shields.io/badge/Spring-6DB33F?style=for-the-badge&logo=spring&logoColor=white">
				<img src="https://img.shields.io/badge/HTML5-E34F26?style=for-the-badge&logo=html5&logoColor=white">
				<img src="https://img.shields.io/badge/Node.js-43853D?style=for-the-badge&logo=node.js&logoColor=white">
				<img src="https://img.shields.io/badge/Rust-orange?style=for-the-badge&logo=rust">
			</div>
			<div class="intro">
				<h1 class="msh1">开发习惯</h1>
				<h2>开发工具</h2>
				<h3><span>MacBook Pro(M3max 10P+4E, 16inch)</span></h3>
				<h3><span>9900X + 192G (4x48G) D5 + Rocky Linux 9.4</span></h3>
				<h2>Linux 偏好：</h2>
				<h3><span>Debian = Rocky Linux > Ubuntu > CentOS > Arch > ....</span>(其中只有RockyLinux与Debian被运用于生产环境)
				</h3>
				<h2>编程语言熟练度排行</h2>
				<h3><span>Java</span> (>= 2.5w行)</h3>
				<h3><span>Kotlin</span>(8k-9k行，仅JVM)</h3>
				<h3>
					<span>JavaScript</span>(2k-3k行)</h3>
				<h3><span>C, C++</span>(2k行)</h3>
			</div>
		</div>
	</div>
</template>

<script>
export default {
	name: 'Title',
	props: {
		msg: String
	},
	computed: {
		msgWithBr() {
			return this.msg ? this.msg.replace(/\n/g, '<br>') : '';
		}
	}
}
</script>

<style scoped>
.intro {
	margin-bottom: 6vh;
}

.github {
	border-radius: 5px;
}

h2 {
	font-weight: 100;
	color: #d1cfcf;
}

@keyframes lighter {
	0% {
		text-shadow: rgba(255, 255, 255, 0.3) 0px 5px 5px, rgba(255, 255, 255, 0.4) 0px -1px 3px, rgba(255, 248, 248, 0.2) 0px 4px 6px, rgba(234, 228, 228, 0.3) 0px 12px 13px, rgba(0, 0, 0, 0.05) 0px -3px 5px;
	}
	50% {
		text-shadow: rgba(201, 198, 198, 0.9) 0px 10px 10px, rgba(210, 201, 201, 0.95) 0px -3px 6px, rgba(255, 248, 248, 0.8) 0px 7px 9px, rgba(234, 228, 228, 0.9) 0px 15px 16px, rgba(0, 0, 0, 0.2) 0px -5px 8px;
	}
	60% {
		text-shadow: rgba(255, 255, 255, 0.3) 0px 5px 5px, rgba(255, 255, 255, 0.4) 0px -1px 3px, rgba(255, 248, 248, 0.2) 0px 4px 6px, rgba(234, 228, 228, 0.3) 0px 12px 13px, rgba(0, 0, 0, 0.05) 0px -3px 5px;
	}
	70% {
		text-shadow: rgba(201, 198, 198, 0.9) 0px 10px 10px, rgba(210, 201, 201, 0.95) 0px -3px 6px, rgba(255, 248, 248, 0.8) 0px 7px 9px, rgba(234, 228, 228, 0.9) 0px 15px 16px, rgba(0, 0, 0, 0.2) 0px -5px 8px;
	}
	100% {
		text-shadow: rgba(255, 255, 255, 0.3) 0px 5px 5px, rgba(255, 255, 255, 0.4) 0px -1px 3px, rgba(255, 248, 248, 0.2) 0px 4px 6px, rgba(234, 228, 228, 0.3) 0px 12px 13px, rgba(0, 0, 0, 0.05) 0px -3px 5px;
	}
}

.light {
	animation: lighter 1s ease-in-out;
}

.quote {
	font-size: 1.4rem;
	margin-left: 5vw;
	font-style: italic;
	font-family: serif;
}

@import "@/css/main.css";
</style>
