<template>
	<div class="peer-metro-card" @click="visitProfile">
		<div class="peer-content">
			<div class="avatar-wrap">
				<img :src="avatar" :alt="name" class="peer-avatar" loading="lazy" />
			</div>
			<div class="peer-info">
				<div class="peer-header font-mono">
					<h3 class="peer-name">{{ name }}</h3>
					<span class="uplink-arrow">&rarr;</span>
				</div>
				<p class="peer-desc">{{ description || '暂无描述信息。' }}</p>
				<span class="peer-url font-mono">{{ displayLink }}</span>
			</div>
		</div>
	</div>
</template>

<script>
export default {
	name: 'FriendsComponent',
	props: {
		avatar: { type: String, default: '' },
		name: { type: String, default: '' },
		description: { type: String, default: '' },
		link: { type: String, default: '' },
	},
	computed: {
		displayLink() {
			if (!this.link) return '';
			try {
				const url = new URL(this.link);
				return url.hostname;
			} catch (e) {
				return this.link;
			}
		},
	},
	methods: {
		visitProfile() {
			if (this.link) {
				window.open(this.link, '_blank');
			}
		},
	},
};
</script>

<style scoped>
.peer-metro-card {
	background: #121212;
	border: 1px solid #262626;
	padding: 28px 32px;
	cursor: pointer;
	overflow: hidden;
	transition: background-color 0.15s ease, border-color 0.15s ease;
}

.peer-metro-card:hover {
	background: #1a1a1a;
	border-color: #ffffff;
}

.peer-content {
	display: flex;
	align-items: flex-start;
	gap: 24px;
}

.avatar-wrap {
	width: 68px;
	height: 68px;
	flex-shrink: 0;
}

.peer-avatar {
	width: 100%;
	height: 100%;
	border: 1px solid #404040;
	object-fit: cover;
	background: #0a0a0a;
	display: block;
}

.peer-info {
	flex-grow: 1;
	min-width: 0;
}

.peer-header {
	display: flex;
	justify-content: space-between;
	align-items: baseline;
	margin-bottom: 10px;
}

.peer-name {
	margin: 0;
	font-size: 1.3rem;
	color: #ffffff;
	font-weight: 500;
	overflow: hidden;
	text-overflow: ellipsis;
	white-space: nowrap;
}

.uplink-arrow {
	color: #ffffff;
	font-size: 1.15rem;
	transition: transform 0.15s ease;
}

.peer-metro-card:hover .uplink-arrow {
	transform: translateX(4px);
}

.peer-desc {
	margin: 0 0 14px;
	font-size: 0.95rem;
	color: #a3a3a3;
	line-height: 1.65;
	display: -webkit-box;
	-webkit-line-clamp: 2;
	-webkit-box-orient: vertical;
	overflow: hidden;
}

.peer-url {
	font-size: 0.82rem;
	color: #737373;
}

.peer-metro-card:hover .peer-url {
	color: #ffffff;
}
</style>
