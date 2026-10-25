<template>
	<header class="apple-header-wrapper" :class="{ 'scrolled': isScrolled }">
		<!-- 1. Apple Global Navigation Bar -->
		<div class="apple-global-nav">
			<div class="global-nav-container">
				<!-- Apple-style Logo -->
				<router-link to="/" class="global-logo" title="Glowstone Newsroom">
					<span class="logo-text">Glowstone</span>
				</router-link>

				<!-- Global Menu Items -->
				<nav class="global-menu">
					<router-link
						v-for="item in navItems"
						:key="item.path"
						:to="item.path"
						class="global-nav-link"
						:class="{ active: isRouteActive(item.path) }"
					>
						{{ item.name }}
					</router-link>
				</nav>

				<!-- Mobile Menu Toggle Button -->
				<button class="mobile-menu-btn" @click="mobileMenuOpen = !mobileMenuOpen" aria-label="Toggle navigation">
					<svg v-if="!mobileMenuOpen" width="18" height="18" viewBox="0 0 18 18" fill="none">
						<path d="M2 4.5h14M2 9h14M2 13.5h14" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>
					</svg>
					<svg v-else width="18" height="18" viewBox="0 0 18 18" fill="none">
						<path d="M4 4l10 10M14 4L4 14" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>
					</svg>
				</button>
			</div>
		</div>

		<!-- 2. Apple Newsroom Sub-Header (Subnav) -->
		<div class="apple-newsroom-subnav">
			<div class="subnav-container">
				<!-- Left: Newsroom Brand -->
				<div class="subnav-brand">
					<router-link to="/" class="newsroom-title">
						Newsroom
					</router-link>
				</div>

				<!-- Right: Quick Filters & Search Pill -->
				<div class="subnav-actions">
					<div class="subnav-categories">
						<router-link to="/archive" class="subnav-cat-link">全部文章</router-link>
						<router-link to="/archive?category=技术" class="subnav-cat-link">技术</router-link>
						<router-link to="/archive?category=闲聊" class="subnav-cat-link">随笔</router-link>
						<router-link to="/about" class="subnav-cat-link">关于萤石</router-link>
					</div>

					<!-- Search Capsule Input -->
					<div class="search-capsule" @click="focusSearch">
						<svg class="search-icon" width="14" height="14" viewBox="0 0 16 16" fill="none">
							<path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z" fill="currentColor"/>
						</svg>
						<input
							ref="searchInput"
							v-model="searchQuery"
							type="text"
							placeholder="Search Newsroom"
							class="search-input"
							@keydown.enter="handleSearch"
						/>
					</div>
				</div>
			</div>
		</div>

		<!-- Mobile Navigation Overlay Drawer -->
		<transition name="apple-drawer">
			<div v-if="mobileMenuOpen" class="mobile-drawer-overlay" @click.self="mobileMenuOpen = false">
				<div class="mobile-drawer-content">
					<div class="drawer-header">
						<span class="drawer-title">Newsroom Menu</span>
						<button class="drawer-close-btn" @click="mobileMenuOpen = false">✕</button>
					</div>
					<nav class="mobile-nav-list">
						<router-link
							v-for="item in navItems"
							:key="'m-' + item.path"
							:to="item.path"
							class="mobile-nav-item"
							:class="{ active: isRouteActive(item.path) }"
							@click="mobileMenuOpen = false"
						>
							{{ item.name }}
						</router-link>
					</nav>
					<div class="drawer-search">
						<input
							v-model="searchQuery"
							type="text"
							placeholder="Search Newsroom..."
							class="mobile-search-input"
							@keydown.enter="handleSearch"
						/>
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
			searchQuery: '',
			navItems: [
				{ path: '/', name: '首页' },
				{ path: '/archive', name: '文章' },
				{ path: '/gallery', name: '画廊' },
				{ path: '/friends', name: '友链' },
				{ path: '/about', name: '关于' },
				{ path: '/announcement', name: '声明' },
			],
		};
	},
	mounted() {
		window.addEventListener('scroll', this.handleScroll, { passive: true });
	},
	beforeUnmount() {
		window.removeEventListener('scroll', this.handleScroll);
	},
	methods: {
		handleScroll() {
			this.isScrolled = window.scrollY > 20;
		},
		isRouteActive(path) {
			if (path === '/') {
				return this.$route.path === '/';
			}
			return this.$route.path.startsWith(path);
		},
		focusSearch() {
			if (this.$refs.searchInput) {
				this.$refs.searchInput.focus();
			}
		},
		handleSearch() {
			if (this.searchQuery.trim()) {
				this.$router.push({
					path: '/archive',
					query: { q: this.searchQuery.trim() }
				});
				this.mobileMenuOpen = false;
			}
		}
	},
};
</script>

<style scoped>
.apple-header-wrapper {
	position: fixed;
	top: 0;
	left: 0;
	width: 100%;
	z-index: 1000;
	transition: all 0.25s ease;
}

/* 1. Global Nav (Apple Dark Nav Bar) */
.apple-global-nav {
	width: 100%;
	height: var(--apple-nav-height, 52px);
	background: rgba(22, 22, 23, 0.92);
	backdrop-filter: saturate(180%) blur(20px);
	-webkit-backdrop-filter: saturate(180%) blur(20px);
	color: #ffffff;
	display: flex;
	align-items: center;
	border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.global-nav-container {
	max-width: var(--apple-max-width, 1240px);
	width: 100%;
	margin: 0 auto;
	padding: 0 24px;
	display: flex;
	align-items: center;
	justify-content: space-between;
}

.global-logo {
	display: flex;
	align-items: center;
	gap: 8px;
	color: #f5f5f7;
	opacity: 0.9;
	transition: opacity 0.15s ease;
	font-weight: 500;
	font-size: 14px;
}

.global-logo:hover {
	opacity: 1;
	color: #ffffff;
}

.apple-icon {
	fill: currentColor;
}

.logo-text {
	letter-spacing: -0.01em;
}

.global-menu {
	display: flex;
	align-items: center;
	gap: 28px;
}

.global-nav-link {
	color: #d2d2d7;
	font-size: 13px;
	font-weight: 400;
	letter-spacing: -0.01em;
	transition: color 0.15s ease;
}

.global-nav-link:hover,
.global-nav-link.active {
	color: #ffffff;
}

.mobile-menu-btn {
	display: none;
	background: none;
	border: none;
	color: #d2d2d7;
	cursor: pointer;
	padding: 6px;
}

/* 2. Apple Newsroom Sub-Header */
.apple-newsroom-subnav {
	width: 100%;
	height: var(--apple-subnav-height, 56px);
	background: rgba(255, 255, 255, 0.88);
	backdrop-filter: saturate(180%) blur(20px);
	-webkit-backdrop-filter: saturate(180%) blur(20px);
	border-bottom: 1px solid var(--apple-border, #e5e5ea);
	display: flex;
	align-items: center;
	transition: box-shadow 0.25s ease, background-color 0.25s ease;
}

.apple-header-wrapper.scrolled .apple-newsroom-subnav {
	box-shadow: 0 4px 16px rgba(0, 0, 0, 0.05);
	background: rgba(255, 255, 255, 0.94);
}

.subnav-container {
	max-width: var(--apple-max-width, 1240px);
	width: 100%;
	margin: 0 auto;
	padding: 0 24px;
	display: flex;
	align-items: center;
	justify-content: space-between;
}

.newsroom-title {
	font-size: 24px;
	font-weight: 700;
	color: var(--apple-text-primary, #1d1d1f);
	letter-spacing: -0.02em;
	transition: opacity 0.15s ease;
}

.newsroom-title:hover {
	opacity: 0.8;
}

.subnav-actions {
	display: flex;
	align-items: center;
	gap: 24px;
}

.subnav-categories {
	display: flex;
	align-items: center;
	gap: 18px;
}

.subnav-cat-link {
	font-size: 13px;
	color: var(--apple-text-secondary, #86868b);
	font-weight: 400;
	transition: color 0.15s ease;
}

.subnav-cat-link:hover {
	color: var(--apple-text-primary, #1d1d1f);
}

/* Search Capsule */
.search-capsule {
	display: flex;
	align-items: center;
	gap: 8px;
	background: #f5f5f7;
	border-radius: var(--apple-radius-pill, 980px);
	padding: 6px 14px;
	transition: background-color 0.2s ease, width 0.2s ease, box-shadow 0.2s ease;
	border: 1px solid transparent;
}

.search-capsule:focus-within {
	background: #ffffff;
	border-color: #d2d2d7;
	box-shadow: 0 0 0 3px rgba(0, 113, 227, 0.15);
}

.search-icon {
	color: #86868b;
	flex-shrink: 0;
}

.search-input {
	border: none;
	background: transparent;
	font-size: 13px;
	color: var(--apple-text-primary, #1d1d1f);
	outline: none;
	width: 120px;
	transition: width 0.2s ease;
}

.search-capsule:focus-within .search-input {
	width: 160px;
}

.search-input::placeholder {
	color: #86868b;
}

/* Mobile Drawer Overlay */
.mobile-drawer-overlay {
	position: fixed;
	top: 0;
	left: 0;
	width: 100vw;
	height: 100vh;
	background: rgba(0, 0, 0, 0.4);
	backdrop-filter: blur(8px);
	z-index: 2000;
	display: flex;
	justify-content: flex-end;
}

.mobile-drawer-content {
	width: 80%;
	max-width: 320px;
	height: 100%;
	background: #ffffff;
	padding: 24px;
	display: flex;
	flex-direction: column;
	gap: 20px;
	box-shadow: -4px 0 24px rgba(0, 0, 0, 0.1);
}

.drawer-header {
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding-bottom: 12px;
	border-bottom: 1px solid #e5e5ea;
}

.drawer-title {
	font-size: 18px;
	font-weight: 700;
	color: #1d1d1f;
}

.drawer-close-btn {
	background: none;
	border: none;
	font-size: 18px;
	color: #86868b;
	cursor: pointer;
}

.mobile-nav-list {
	display: flex;
	flex-direction: column;
	gap: 16px;
}

.mobile-nav-item {
	font-size: 16px;
	font-weight: 500;
	color: #1d1d1f;
	padding: 8px 0;
}

.mobile-nav-item.active {
	color: var(--apple-accent, #0071e3);
}

.mobile-search-input {
	width: 100%;
	padding: 10px 14px;
	border-radius: 980px;
	border: 1px solid #e5e5ea;
	background: #f5f5f7;
	font-size: 14px;
	outline: none;
}

.apple-drawer-enter-active,
.apple-drawer-leave-active {
	transition: opacity 0.25s ease, transform 0.25s ease;
}

.apple-drawer-enter-from,
.apple-drawer-leave-to {
	opacity: 0;
	transform: translateX(100%);
}

@media screen and (max-width: 860px) {
	.global-menu {
		display: none;
	}
	.mobile-menu-btn {
		display: block;
	}
	.subnav-categories {
		display: none;
	}
	.search-capsule {
		padding: 5px 10px;
	}
	.search-input {
		width: 100px;
	}
	.search-capsule:focus-within .search-input {
		width: 120px;
	}
}
</style>
