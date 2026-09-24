<template>
	<div class="marquee-bar font-mono" :class="{ 'invert-bar': invert }">
		<div class="bar-edge-mark left-mark">+</div>
		<div class="marquee-viewport">
			<div class="marquee-inner">
				<div class="marquee-sequence">
					<span v-for="i in 3" :key="i" class="marquee-segment">
						{{ text || defaultText }}
					</span>
				</div>
				<div class="marquee-sequence" aria-hidden="true">
					<span v-for="i in 3" :key="'dup-' + i" class="marquee-segment">
						{{ text || defaultText }}
					</span>
				</div>
			</div>
		</div>
		<div class="bar-edge-mark right-mark">+</div>
	</div>
</template>

<script>
export default {
	name: 'AsciiMarquee',
	props: {
		text: {
			type: String,
			default: '',
		},
		invert: {
			type: Boolean,
			default: false,
		},
	},
	data() {
		return {
			defaultText: 'GLOWSTONE // JVM & SYSTEM ARCHITECTURE // LINUX & POSIX RUNTIME // PINK FLOYD & LED ZEPPELIN // GLANCE & GO // 2026 //',
		};
	},
};
</script>

<style scoped>
.marquee-bar {
	position: relative;
	width: 100%;
	height: 38px;
	background: #0d0d0d;
	border-top: 1px solid #262626;
	border-bottom: 1px solid #262626;
	overflow: hidden;
	display: flex;
	align-items: center;
	margin: 50px 0;
	user-select: none;
}

.bar-edge-mark {
	position: absolute;
	top: 50%;
	transform: translateY(-50%);
	font-size: 0.95rem;
	color: #666666;
	z-index: 3;
	padding: 0 16px;
	background: #0d0d0d;
	pointer-events: none;
}

.left-mark {
	left: 0;
}

.right-mark {
	right: 0;
}

.marquee-viewport {
	width: 100%;
	overflow: hidden;
	display: flex;
}

.marquee-inner {
	display: flex;
	width: max-content;
	animation: scroll-marquee 28s linear infinite;
}

.marquee-bar:hover .marquee-inner {
	animation-play-state: paused;
}

.marquee-sequence {
	display: flex;
	flex-shrink: 0;
	white-space: nowrap;
}

.marquee-segment {
	font-size: 0.82rem;
	letter-spacing: 2px;
	color: #888888;
	padding-right: 32px;
	transition: color 0.15s ease;
}

.marquee-bar:hover .marquee-segment {
	color: #ffffff;
}

.invert-bar {
	background: #171717;
	border-color: #383838;
}

@keyframes scroll-marquee {
	0% {
		transform: translateX(0);
	}
	100% {
		transform: translateX(-50%);
	}
}
</style>
