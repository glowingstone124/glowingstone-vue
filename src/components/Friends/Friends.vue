<template>
	<div class="friends-bento-container">
		<!-- Section Header -->
		<div class="section-header">
			<h1 class="section-title">友链 <span class="sub">Friends</span></h1>
			<span class="section-meta font-mono">共 {{ peerList.length }} 个节点</span>
		</div>

		<!-- Protocol Info Bento Tile (Span 12) -->
		<div class="metro-tile handshake-tile">
			<div class="handshake-grid font-mono">
				<!-- Left: How to connect -->
				<div class="handshake-col">
					<div class="col-head">[ 申请友链 / Connect ]</div>
					<p class="col-text">
						欢迎交换友链！请在群内直接 @ 我或通过邮箱发送以下节点信息：
					</p>
					<ul class="spec-ul">
						<li><span class="bullet">&gt;</span> 博客网址 (Site URL)</li>
						<li><span class="bullet">&gt;</span> 博客名称 (Site Name)</li>
						<li><span class="bullet">&gt;</span> 节点徽标 (Avatar / Logo URL)</li>
						<li><span class="bullet">&gt;</span> 简介说明 (Site Description)</li>
					</ul>
				</div>

				<!-- Right: My Info with Copy -->
				<div class="handshake-col right-col">
					<div class="col-head">[ 本站信息 / Local Node ]</div>
					<div class="metadata-block">
						<div class="meta-row">
							<span class="m-key">网址:</span>
							<span class="m-val">https://glowingstone.cn</span>
						</div>
						<div class="meta-row">
							<span class="m-key">名字:</span>
							<span class="m-val">Glowstone 萤石</span>
						</div>
						<div class="meta-row">
							<span class="m-key">图标:</span>
							<span class="m-val text-dim">https://avatars.githubusercontent.com/u/163746181</span>
						</div>
						<div class="meta-row">
							<span class="m-key">介绍:</span>
							<span class="m-val">每月更新的可能性为lim -> 0，jvm痴子，全栈小白，性能优化魔怔人。</span>
						</div>
					</div>
					<button class="copy-meta-btn font-mono" @click="copyMetadata">
						{{ copyStatus ? '[✓ 已复制配置信息]' : '[复制节点信息 JSON]' }}
					</button>
				</div>
			</div>
		</div>

		<!-- Peer Nodes Metro Grid -->
		<div class="peers-bento-grid">
			<FriendsComponent
				v-for="peer in peerList"
				:key="peer.name"
				:avatar="peer.avatar"
				:name="peer.name"
				:description="peer.description"
				:link="peer.link"
			/>
		</div>

		<VisitCounter />
	</div>
</template>

<script>
import VisitCounter from "@/components/VisitCounter.vue";
import FriendsComponent from './FriendsComponent.vue';

export default {
	name: 'Friends',
	components: {
		VisitCounter,
		FriendsComponent,
	},
	data() {
		return {
			copyStatus: false,
			peerList: [
				{
					avatar: 'https://minecreeper.top/images/avatar.png',
					name: '矿井小怕的茶馆',
					description: '也许是自己的后院，但更是大家的茶馆。',
					link: 'https://minecreeper.top/',
				},
				{
					avatar: 'https://serinanya.cn/assets-zone-1/avatar-round.png',
					name: 'SerinaNya',
					description: 'Serina 的日常 / daily of SerinaNya',
					link: 'https://serinanya.cn',
				},
				{
					avatar: 'https://q2.qlogo.cn/headimg_dl?dst_uin=2504726488&spec=100',
					name: '凌空的博客小站',
					description: '分享技术与生活的点点滴滴。',
					link: 'https://www.skyblogs.xyz/',
				},
				{
					avatar: 'https://blog.ibctech.top/usr/themes/handsome/assets/img/tx.jpg',
					name: 'NLのBlog',
					description: '技术探索与架构经验随笔。',
					link: 'https://blog.ibctech.top',
				},
				{
					avatar: 'https://picture.zhanganzhi.com/common/avatar.jpg',
					name: "Andy's Blog",
					description: '热爱探索全栈开发与新技术的工程师博客。',
					link: 'https://blog.zhanganzhi.com/',
				},
				{
					avatar: 'https://static.xhustudio.eu.org/images/jpeg/102411014.jpg',
					name: 'SЕSSのB10GТЕ5Т',
					description: 'Session的个人博客, 这里有各种类型的有趣的文章内容。',
					link: 'https://sess.xhustudio.eu.org/',
				},
			],
		};
	},
	methods: {
		copyMetadata() {
			const meta = {
				name: "Glowstone 萤石",
				url: "https://glowingstone.cn",
				avatar: "https://avatars.githubusercontent.com/u/163746181",
				description: "每月更新的可能性为lim -> 0，jvm痴子，全栈小白，性能优化魔怔人。",
			};
			navigator.clipboard.writeText(JSON.stringify(meta, null, 2)).then(() => {
				this.copyStatus = true;
				setTimeout(() => {
					this.copyStatus = false;
				}, 2000);
			});
		},
	},
};
</script>

<style scoped>
.friends-bento-container {
	max-width: var(--max-content-width);
	margin: 0 auto 120px;
	padding: 0 40px;
	box-sizing: border-box;
}

/* Handshake Tile */
.handshake-tile {
	margin-bottom: 44px;
	border: 1px solid #262626;
	background: #121212;
	padding: 44px 48px;
}

.handshake-grid {
	display: grid;
	grid-template-columns: 1fr 1fr;
	gap: 48px;
}

.col-head {
	font-size: 1rem;
	color: #ffffff;
	margin-bottom: 16px;
	font-weight: 500;
}

.col-text {
	font-size: 0.95rem;
	color: #d4d4d4;
	line-height: 1.75;
	margin: 0 0 20px;
}

.spec-ul {
	list-style: none;
	padding: 0;
	margin: 0;
	display: flex;
	flex-direction: column;
	gap: 10px;
	font-size: 0.9rem;
	color: #ffffff;
}

.bullet {
	color: #a3a3a3;
	margin-right: 10px;
}

.metadata-block {
	background: #0a0a0a;
	border: 1px solid #262626;
	padding: 20px 24px;
	display: flex;
	flex-direction: column;
	gap: 12px;
	margin-bottom: 20px;
}

.meta-row {
	display: flex;
	gap: 12px;
	font-size: 0.85rem;
}

.m-key {
	color: #a3a3a3;
	min-width: 60px;
}

.m-val {
	color: #ffffff;
}

.text-dim {
	color: #737373;
	word-break: break-all;
}

.copy-meta-btn {
	background: #0a0a0a;
	border: 1px solid #404040;
	color: #ffffff;
	padding: 12px 20px;
	cursor: pointer;
	font-size: 0.85rem;
	transition: all 0.15s ease;
	width: 100%;
}

.copy-meta-btn:hover {
	background: #ffffff;
	color: #000000;
	border-color: #ffffff;
}

/* Peers Bento Grid */
.peers-bento-grid {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(440px, 1fr));
	gap: 28px;
	margin-bottom: 40px;
}

@media screen and (max-width: 768px) {
	.handshake-grid {
		grid-template-columns: 1fr;
		gap: 28px;
	}
	.peers-bento-grid {
		grid-template-columns: 1fr;
	}
	.friends-bento-container {
		padding: 0 20px;
	}
	.handshake-tile {
		padding: 28px 24px;
	}
}
</style>
