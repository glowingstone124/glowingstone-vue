<template>
	<header class="metro-navbar" :class="{ 'scrolled': isScrolled }">
		<div class="nav-container">
			<!-- Brand -->
			<router-link to="/" class="nav-brand">
				<img src="https://avatars.githubusercontent.com/u/163746181" alt="Logo" class="nav-logo" />
				<span class="brand-title">Glowstone <span class="brand-sub">萤石</span></span>
			</router-link>

			<!-- Desktop Metro Navigation Links -->
			<nav class="desktop-nav font-mono">
				<router-link
					v-for="item in navItems"
					:key="item.path"
					:to="item.path"
					class="metro-nav-item"
					:class="{ active: isRouteActive(item.path) }"
					@mouseenter="triggerScramble($event, item.name)"
				>
					<span class="bracket">[</span>
					<span class="nav-text">{{ item.name }}</span>
					<span class="bracket">]</span>
				</router-link>
			</nav>

			<!-- Terminal HUD Clock & Status -->
			<div class="nav-hud font-mono">
				<span class="hud-status-badge">
					<span class="hud-dot"></span>SYS:ONLINE
				</span>
				<span class="hud-sep">//</span>
				<span class="hud-clock">{{ currentTime }}</span>
			</div>

			<!-- Mobile Toggle -->
			<button class="mobile-toggle-btn font-mono" @click="mobileMenuOpen = !mobileMenuOpen" aria-label="Menu">
				{{ mobileMenuOpen ? '[✕ 关]' : '[☰ 单]' }}
			</button>
		</div>

		<!-- Mobile Navigation Drawer -->
		<transition name="drawer-fade">
			<div v-if="mobileMenuOpen" class="mobile-drawer" @click.self="mobileMenuOpen = false">
				<div class="drawer-panel font-mono">
					<div class="drawer-header">
						<span>// 导航目录</span>
						<button class="drawer-close" @click="mobileMenuOpen = false">[✕]</button>
					</div>
					<div class="drawer-nav-list">
						<router-link
							v-for="item in navItems"
							:key="'m-' + item.path"
							:to="item.path"
							class="drawer-nav-item"
							:class="{ active: isRouteActive(item.path) }"
							@click="mobileMenuOpen = false"
						>
							<span class="item-name">{{ item.name }}</span>
							<span class="item-path">{{ item.path }}</span>
						</router-link>
					</div>
				</div>
			</div>
		</transition>
	</header>
</template>

<script>
export default {
	name: 'Nav',
	data() {
		return {
			isScrolled: false,
			mobileMenuOpen: false,
			currentTime: '',
			clockTimer: null,
			navItems: [
				{ path: '/', name: '主页' },
				{ path: '/archive', name: '文章' },
				{ path: '/friends', name: '友链' },
				{ path: '/gallery', name: '画廊' },
				{ path: '/about', name: '关于' },
				{ path: '/announcement', name: '特别声明' },
			],
		};
	},
	mounted() {
		window.addEventListener('scroll', this.handleScroll, { passive: true });
		this.updateClock();
		this.clockTimer = setInterval(this.updateClock, 1000);
	},
	beforeUnmount() {
		window.removeEventListener('scroll', this.handleScroll);
		if (this.clockTimer) clearInterval(this.clockTimer);
	},
	methods: {
		updateClock() {
			const now = new Date();
			const h = String(now.getHours()).padStart(2, '0');
			const m = String(now.getMinutes()).padStart(2, '0');
			const s = String(now.getSeconds()).padStart(2, '0');
			this.currentTime = `${h}:${m}:${s}`;
		},
		handleScroll() {
			this.isScrolled = window.scrollY > 15;
		},
		isRouteActive(path) {
			if (path === '/') return this.$route.path === '/';
			return this.$route.path.startsWith(path);
		},
		triggerScramble(event, originalText) {
			const el = event.currentTarget.querySelector('.nav-text');
			if (!el || el._scrambling) return;
			el._scrambling = true;
			const chars = '01#%&*?!/[]_—+<>';
			let frame = 0;
			const totalFrames = 8;
			const interval = setInterval(() => {
				frame++;
				if (frame >= totalFrames) {
					clearInterval(interval);
					el.innerText = originalText;
					el._scrambling = false;
				} else {
					el.innerText = originalText
						.split('')
						.map((c, i) => (frame > i * 2 ? c : chars[Math.floor(Math.random() * chars.length)]))
						.join('');
				}
			}, 24);
		},
	},
};
</script>

<style scoped>
.metro-navbar {
	position: fixed;
	top: 0;
	left: 0;
	width: 100%;
	height: 64px;
	z-index: 1000;
	background: #0a0a0a;
	border-bottom: 1px solid #262626;
	transition: border-color 0.15s ease;
}

.metro-navbar.scrolled {
	background: #000000;
	border-bottom-color: #404040;
}

.nav-container {
	max-width: var(--max-content-width);
	margin: 0 auto;
	height: 100%;
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding: 0 32px;
	box-sizing: border-box;
}

/* Brand */
.nav-brand {
	display: flex;
	align-items: center;
	gap: 12px;
	text-decoration: none;
	color: #ffffff;
}

.nav-logo {
	width: 32px;
	height: 32px;
	border: 1px solid #404040;
	object-fit: cover;
	display: block;
}

.brand-title {
	font-size: 1.15rem;
	font-weight: 500;
	letter-spacing: -0.3px;
}

.brand-sub {
	color: #a3a3a3;
	font-weight: 300;
	margin-left: 4px;
}

/* Desktop Navigation */
.desktop-nav {
	display: flex;
	align-items: center;
	gap: 4px;
}

.metro-nav-item {
	display: inline-flex;
	align-items: center;
	gap: 4px;
	padding: 8px 16px;
	text-decoration: none;
	color: #a3a3a3;
	font-size: 0.88rem;
	border: 1px solid transparent;
	background: transparent;
	transition: all 0.15s ease;
}

.metro-nav-item .bracket {
	color: #525252;
}

.metro-nav-item:hover {
	color: #ffffff;
	background: #171717;
	border-color: #404040;
}

.metro-nav-item.active {
	color: #000000;
	background: #ffffff;
	border-color: #ffffff;
	font-weight: 500;
}

/* Terminal HUD Clock & Status */
.nav-hud {
	display: flex;
	align-items: center;
	gap: 10px;
	font-size: 0.8rem;
	color: #737373;
	border-left: 1px solid #262626;
	padding-left: 16px;
	margin-left: 8px;
	user-select: none;
}

.hud-status-badge {
	display: inline-flex;
	align-items: center;
	gap: 6px;
	color: #a3a3a3;
	letter-spacing: 0.5px;
}

.hud-dot {
	width: 5px;
	height: 5px;
	background: #ffffff;
	display: inline-block;
	animation: hud-blink 1.8s infinite ease-in-out;
}

.hud-sep {
	color: #404040;
}

.hud-clock {
	color: #ffffff;
	letter-spacing: 1px;
}

@keyframes hud-blink {
	0%, 100% { opacity: 1; }
	50% { opacity: 0.25; }
}

.metro-nav-item.active .bracket {
	color: #000000;
}

/* Mobile Toggle */
.mobile-toggle-btn {
	display: none;
	background: #121212;
	border: 1px solid #404040;
	color: #ffffff;
	padding: 6px 12px;
	cursor: pointer;
	font-size: 0.85rem;
}

.mobile-toggle-btn:hover {
	background: #262626;
}

/* Mobile Drawer */
.mobile-drawer {
	position: fixed;
	top: 64px;
	left: 0;
	width: 100%;
	height: calc(100vh - 64px);
	background: rgba(0, 0, 0, 0.95);
	z-index: 999;
	padding: 24px;
	box-sizing: border-box;
}

.drawer-panel {
	max-width: 500px;
	margin: 0 auto;
	background: #121212;
	border: 1px solid #404040;
	padding: 20px;
}

.drawer-header {
	display: flex;
	justify-content: space-between;
	align-items: center;
	color: #a3a3a3;
	font-size: 0.85rem;
	padding-bottom: 14px;
	border-bottom: 1px solid #262626;
	margin-bottom: 14px;
}

.drawer-close {
	background: transparent;
	border: none;
	color: #ffffff;
	cursor: pointer;
	font-family: inherit;
	font-size: 0.9rem;
}

.drawer-nav-list {
	display: flex;
	flex-direction: column;
	gap: 8px;
}

.drawer-nav-item {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 14px 16px;
	background: #0a0a0a;
	border: 1px solid #262626;
	text-decoration: none;
	color: #ffffff;
	font-size: 0.95rem;
}

.drawer-nav-item.active {
	background: #ffffff;
	color: #000000;
	border-color: #ffffff;
}

.drawer-nav-item .item-path {
	color: #737373;
	font-size: 0.78rem;
}

.drawer-nav-item.active .item-path {
	color: #525252;
}

.drawer-fade-enter-active,
.drawer-fade-leave-active {
	transition: opacity 0.15s ease;
}

.drawer-fade-enter-from,
.drawer-fade-leave-to {
	opacity: 0;
}

@media screen and (max-width: 820px) {
	.desktop-nav,
	.nav-hud {
		display: none;
	}
	.mobile-toggle-btn {
		display: block;
	}
}
</style>
