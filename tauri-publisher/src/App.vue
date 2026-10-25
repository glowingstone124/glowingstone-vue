<template>
	<div class="macos-app-window">
		<!-- macOS Sidebar Navigation -->
		<aside class="macos-sidebar">
			<!-- Window Traffic Light Area / App Header -->
			<div class="sidebar-header">
				<div class="app-brand-pill">
					<svg class="apple-logo-svg" viewBox="0 0 170 170" width="16" height="16" fill="currentColor">
						<path d="M150.37 130.25c-2.45 5.66-5.35 10.87-8.71 15.66-4.58 6.53-8.33 11.05-11.22 13.56-4.48 4.12-9.28 6.23-14.42 6.35-3.69 0-8.14-1.05-13.32-3.18-5.19-2.12-9.97-3.17-14.34-3.17-4.58 0-9.49 1.05-14.75 3.17-5.26 2.13-9.5 3.24-12.74 3.35-4.35.13-9.16-1.9-14.42-6.08-3.7-3.04-7.6-7.7-11.71-13.98-5.87-8.91-10.45-18.91-13.74-30.01-3.29-11.1-4.94-21.75-4.94-31.95 0-14.99 3.99-27.17 11.96-36.54 7.97-9.37 17.84-14.15 29.6-14.35 4.79 0 10.22 1.25 16.3 3.75 6.08 2.5 10.3 3.82 12.67 3.96 1.95-.14 6.38-1.53 13.3-4.17 6.91-2.64 12.62-3.8 17.13-3.48 12.72.63 22.88 5.48 30.48 14.56-11.08 6.74-16.51 16.03-16.3 27.87.21 9.46 3.8 17.38 10.77 23.77 6.96 6.39 15.34 10.02 25.13 10.88-2.61 7.92-5.76 15.75-9.45 23.49zM119.22 31.02c0-7.28 2.66-13.99 7.99-20.13 5.33-6.14 11.83-9.97 19.5-11.49.22 1.09.33 2.1.33 3.04 0 7.28-2.73 14.07-8.18 20.36-5.45 6.29-12.01 10.06-19.64 11.31-.21-.76-.32-1.79-.32-3.09z"/>
					</svg>
					<span class="app-brand-name">Newsroom Publisher</span>
				</div>
			</div>

			<!-- Navigation Menu Items -->
			<nav class="sidebar-nav">
				<button
					class="nav-btn"
					:class="{ active: currentTab === 'editor' }"
					@click="currentTab = 'editor'"
				>
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M12 20h9"></path>
						<path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"></path>
					</svg>
					<span>撰写文章</span>
				</button>

				<button
					class="nav-btn"
					:class="{ active: currentTab === 'library' }"
					@click="loadArticlesList(); currentTab = 'library'"
				>
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
						<path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
					</svg>
					<span>文章管理</span>
					<span class="nav-count-badge">{{ articles.length }}</span>
				</button>

				<button
					class="nav-btn"
					:class="{ active: currentTab === 'publish' }"
					@click="loadGitStatus(); currentTab = 'publish'"
				>
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10"></circle>
						<polygon points="12 8 8 12 11 12 11 16 13 16 13 12 16 12 12 8"></polygon>
					</svg>
					<span>发布与部署</span>
					<span v-if="hasGitChanges" class="git-dot"></span>
				</button>

				<button
					class="nav-btn"
					:class="{ active: currentTab === 'settings' }"
					@click="currentTab = 'settings'"
				>
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="3"></circle>
						<path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
					</svg>
					<span>工作区设置</span>
				</button>
			</nav>

			<!-- Bottom Status Widget -->
			<div class="sidebar-footer">
				<div class="repo-meta-box">
					<div class="meta-label">当前分支: <strong>{{ repoInfo.git_branch || 'main' }}</strong></div>
					<div class="meta-articles">文章总数: {{ repoInfo.article_count || 0 }} 篇</div>
				</div>

				<button class="preview-web-btn" @click="openBrowser('http://localhost:8080')">
					<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10"></circle>
						<line x1="2" y1="12" x2="22" y2="12"></line>
						<path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>
					</svg>
					本地博客预览
				</button>
			</div>
		</aside>

		<!-- Main Workspace Area -->
		<main class="macos-workspace">
			<!-- TOP TOOLBAR -->
			<header class="workspace-toolbar">
				<div class="toolbar-left">
					<h2 class="toolbar-title">{{ getTabTitle() }}</h2>
				</div>

				<div class="toolbar-center" v-if="currentTab === 'editor'">
					<!-- View Segmented Control -->
					<div class="segmented-control">
						<button
							class="segment-btn"
							:class="{ active: editorView === 'split' }"
							@click="editorView = 'split'"
						>
							分栏对照
						</button>
						<button
							class="segment-btn"
							:class="{ active: editorView === 'edit' }"
							@click="editorView = 'edit'"
						>
							仅编辑
						</button>
						<button
							class="segment-btn"
							:class="{ active: editorView === 'preview' }"
							@click="editorView = 'preview'"
						>
							新闻室预览
						</button>
					</div>
				</div>

				<div class="toolbar-right" v-if="currentTab === 'editor'">
					<button class="apple-btn-secondary" @click="resetEditor">
						新建空白
					</button>
					<button class="apple-btn-secondary" @click="handleSaveArticle(false)">
						仅保存本地
					</button>
					<button class="apple-btn-primary" @click="handleSaveArticle(true)">
						保存并推送 🚀
					</button>
				</div>
			</header>

			<!-- TAB 1: ARTICLE EDITOR -->
			<div v-show="currentTab === 'editor'" class="tab-pane editor-tab-pane">
				<!-- Article Meta Card -->
				<section class="meta-card">
					<div class="meta-row-1">
						<input
							v-model="articleForm.title"
							type="text"
							placeholder="输入文章标题 (Title)..."
							class="title-input"
							@input="autoGenerateFilename"
						/>
					</div>

					<div class="meta-grid">
						<!-- Category Selector -->
						<div class="field-group">
							<label class="field-label">文章类别 (Category)</label>
							<div class="category-segmented">
								<button
									v-for="(name, idx) in ['技术', '闲聊', '时事', '文章/小说']"
									:key="idx"
									class="cat-segment-btn"
									:class="{ active: articleForm.category === idx }"
									@click="articleForm.category = idx"
								>
									{{ name }}
								</button>
							</div>
						</div>

						<!-- Filename -->
						<div class="field-group">
							<label class="field-label">文件名称 (Markdown Slug)</label>
							<input
								v-model="articleForm.filename"
								type="text"
								placeholder="e.g. 2026-new-feature.md"
								class="apple-text-input"
							/>
						</div>

						<!-- Date -->
						<div class="field-group">
							<label class="field-label">发布日期 (Date)</label>
							<input
								v-model="articleForm.date"
								type="text"
								placeholder="2026年10月25日"
								class="apple-text-input"
							/>
						</div>

						<!-- AI Switch -->
						<div class="field-group switch-group">
							<label class="field-label">AI 辅助内容</label>
							<label class="apple-switch">
								<input type="checkbox" v-model="articleForm.ai_generated" />
								<span class="switch-slider"></span>
							</label>
						</div>
					</div>

					<!-- Abstract -->
					<div class="field-group mt-12">
						<label class="field-label">文章摘要与简介 (Abstract)</label>
						<textarea
							v-model="articleForm.abstract_text"
							rows="2"
							placeholder="随手写下一两句精炼的摘要描述，呈现于 Apple Newsroom 首页卡片..."
							class="apple-textarea"
						></textarea>
					</div>

					<!-- Cover Image Picker -->
					<div class="field-group mt-12">
						<label class="field-label">封面图片 (Cover Picture)</label>
						<div class="cover-picker-row">
							<input
								v-model="articleForm.picture"
								type="text"
								placeholder="图片 URL、本地路径或拖拽上传下方区域..."
								class="apple-text-input flex-1"
							/>
							<label class="upload-btn-label">
								<input type="file" accept="image/*" class="hidden-file-input" @change="handleImageUpload" />
								本地上传
							</label>
						</div>

						<div v-if="articleForm.picture && articleForm.picture !== 'none'" class="cover-preview-box">
							<img :src="getCoverSrc(articleForm.picture)" alt="Cover" class="cover-preview-img" />
							<button class="remove-cover-btn" @click="articleForm.picture = 'none'">✕ 移除封面</button>
						</div>
					</div>
				</section>

				<!-- Editor & Preview Body -->
				<section class="editor-container" :class="editorView">
					<!-- Left: Markdown Source Editor -->
					<div v-show="editorView !== 'preview'" class="editor-pane">
						<!-- Markdown Toolbar Shortcuts -->
						<div class="md-toolbar">
							<button class="tool-btn" @click="insertText('**', '**')"><strong>B</strong></button>
							<button class="tool-btn" @click="insertText('*', '*')"><em>I</em></button>
							<button class="tool-btn" @click="insertText('## ')">H2</button>
							<button class="tool-btn" @click="insertText('### ')">H3</button>
							<button class="tool-btn" @click="insertText('> ')">引用</button>
							<button class="tool-btn" @click="insertText('```rust\n', '\n```')">&lt;/&gt;</button>
							<button class="tool-btn" @click="insertText('`', '`')">行内代码</button>
							<button class="tool-btn" @click="insertText('$ ', ' $')">公式</button>
							<button class="tool-btn" @click="insertText('[标题](', ')')">链接</button>
							<button class="tool-btn" @click="insertText('![说明](', ')')">插入图片</button>
						</div>

						<textarea
							ref="markdownTextarea"
							v-model="articleForm.content"
							class="code-editor-textarea"
							placeholder="在此处以 Markdown 格式撰写您的文章正文..."
							@keydown.tab.prevent="insertTab"
						></textarea>
					</div>

					<!-- Right: Live Apple Newsroom Preview -->
					<div v-show="editorView !== 'edit'" class="preview-pane">
						<div class="apple-preview-wrapper">
							<div class="preview-header-meta">
								<span class="preview-badge">{{ getCategoryName(articleForm.category) }}</span>
								<span class="preview-dot">·</span>
								<span class="preview-date">{{ articleForm.date || '2026年10月' }}</span>
								<span v-if="articleForm.ai_generated" class="preview-ai-pill">AI 辅助内容</span>
							</div>

							<h1 class="preview-article-title">{{ articleForm.title || '无标题文章' }}</h1>

							<div class="preview-meta-bar">
								<span>Glowingstone 萤石</span>
								<span class="preview-dot">·</span>
								<span>{{ articleForm.content.length }} 字符</span>
							</div>

							<div v-if="articleForm.picture && articleForm.picture !== 'none'" class="preview-cover-banner">
								<img :src="getCoverSrc(articleForm.picture)" alt="Cover Banner" class="banner-img" />
							</div>

							<div class="preview-html-body" v-html="renderedMarkdown"></div>
						</div>
					</div>
				</section>
			</div>

			<!-- TAB 2: ARTICLE LIBRARY -->
			<div v-show="currentTab === 'library'" class="tab-pane library-tab-pane">
				<div class="library-search-bar">
					<input
						v-model="librarySearch"
						type="text"
						placeholder="搜索文章标题或关键词..."
						class="library-search-input"
					/>
					<div class="library-count-tag">
						共 {{ filteredArticles.length }} 篇文章
					</div>
				</div>

				<div class="articles-table-wrapper">
					<table class="apple-table">
						<thead>
							<tr>
								<th>封面</th>
								<th>标题</th>
								<th>分类</th>
								<th>文件名</th>
								<th>发布日期</th>
								<th class="text-right">操作</th>
							</tr>
						</thead>
						<tbody>
							<tr v-for="item in filteredArticles" :key="item.filename">
								<td class="thumb-cell">
									<img
										v-if="item.picture && item.picture !== 'none'"
										:src="getCoverSrc(item.picture)"
										class="table-thumb"
									/>
									<div v-else class="table-thumb-empty">无</div>
								</td>
								<td class="title-cell">
									<div class="table-article-title">{{ item.title }}</div>
									<div class="table-article-abstract">{{ item.abstract_text }}</div>
								</td>
								<td>
									<span class="table-cat-badge">{{ item.category_name }}</span>
								</td>
								<td class="filename-cell">{{ item.filename }}</td>
								<td>{{ item.date }}</td>
								<td class="actions-cell text-right">
									<button class="tbl-action-btn primary" @click="loadArticleToEditor(item.filename)">
										编辑
									</button>
									<button class="tbl-action-btn" @click="openArticleInWeb(item.filename)">
										预览
									</button>
									<button class="tbl-action-btn danger" @click="handleDeleteArticle(item.filename)">
										删除
									</button>
								</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>

			<!-- TAB 3: PUBLISH & DEPLOY -->
			<div v-show="currentTab === 'publish'" class="tab-pane publish-tab-pane">
				<div class="publish-card">
					<h3 class="publish-title">Git 一键部署到 GitHub &amp; Pages</h3>
					<p class="publish-desc">
						通过调用底层 Git 自动化执行 <code>git add .</code>、<code>git commit</code> 与 <code>git push origin main</code>。
					</p>

					<div class="commit-input-box">
						<label class="field-label">Git Commit Message</label>
						<input
							v-model="commitMessage"
							type="text"
							placeholder="publish: updates from Glowstone Publisher"
							class="apple-text-input"
						/>
					</div>

					<div class="git-status-preview">
						<div class="status-preview-header">
							<span>未提交变更清单 (git status)</span>
							<button class="refresh-status-btn" @click="loadGitStatus">刷新状态</button>
						</div>
						<pre class="git-status-output">{{ gitStatusOutput || '工作区干净，无未提交更改。' }}</pre>
					</div>

					<div class="publish-action-row">
						<button
							class="apple-btn-primary publish-big-btn"
							:disabled="isPublishing"
							@click="handleGitDeploy"
						>
							<span v-if="!isPublishing">立即推送到 GitHub (origin/main) 🚀</span>
							<span v-else>正在推送中，请稍候...</span>
						</button>
					</div>

					<!-- Git Output Console -->
					<div v-if="gitLogOutput" class="git-log-console">
						<div class="console-header">
							<span>执行结果与推送日志</span>
							<span :class="gitSuccess ? 'status-ok' : 'status-err'">
								{{ gitSuccess ? '✓ 推送成功' : '✕ 推送失败或警告' }}
							</span>
						</div>
						<pre class="console-body">{{ gitLogOutput }}</pre>
					</div>
				</div>
			</div>

			<!-- TAB 4: SETTINGS -->
			<div v-show="currentTab === 'settings'" class="tab-pane settings-tab-pane">
				<div class="settings-card">
					<h3 class="settings-title">工作区与博客配置</h3>

					<div class="settings-item">
						<label class="field-label">博客仓库绝对路径</label>
						<input
							v-model="customRepoPath"
							type="text"
							class="apple-text-input"
							@change="saveSettings"
						/>
						<p class="settings-hint">默认自动识别 <code>/Users/glowingstone/WebstormProjects/glowingstone-vue</code>。</p>
					</div>

					<div class="settings-item">
						<label class="field-label">本地开发服务地址</label>
						<input
							v-model="devServerUrl"
							type="text"
							class="apple-text-input"
						/>
					</div>

					<div class="settings-actions">
						<button class="apple-btn-primary" @click="refreshRepo">保存并重新连接仓库</button>
					</div>
				</div>
			</div>
		</main>
	</div>
</template>

<script>
import { ref, computed, onMounted } from 'vue';
import { marked } from 'marked';

export default {
	name: 'App',
	setup() {
		const currentTab = ref('editor');
		const editorView = ref('split');
		const isPublishing = ref(false);

		const repoInfo = ref({
			path: '',
			article_count: 0,
			git_branch: 'main',
			git_status: '',
		});

		const articles = ref([]);
		const librarySearch = ref('');
		const customRepoPath = ref('/Users/glowingstone/WebstormProjects/glowingstone-vue');
		const devServerUrl = ref('http://localhost:8080');

		const commitMessage = ref('publish: new article from publisher');
		const gitStatusOutput = ref('');
		const gitLogOutput = ref('');
		const gitSuccess = ref(false);

		const articleForm = ref({
			filename: '',
			title: '',
			category: 0,
			abstract_text: '',
			picture: 'none',
			ai_generated: false,
			date: '2026年10月25日',
			content: '# 开始撰写文章\n\n在此输入您的文章正文...',
		});

		const markdownTextarea = ref(null);

		// Tauri invoke wrapper
		const invokeTauri = async (cmd, args = {}) => {
			if (window.__TAURI__ && window.__TAURI__.invoke) {
				return await window.__TAURI__.invoke(cmd, args);
			}
			console.warn(`[Tauri Mock] Invoke '${cmd}'`, args);
			return null;
		};

		const loadRepoInfo = async () => {
			try {
				const info = await invokeTauri('get_repo_info', {
					customPath: customRepoPath.value || null,
				});
				if (info) {
					repoInfo.value = info;
					if (info.path) {
						customRepoPath.value = info.path;
					}
				}
			} catch (err) {
				console.error('Failed to load repo info:', err);
			}
		};

		const loadArticlesList = async () => {
			try {
				const list = await invokeTauri('list_articles', {
					customPath: customRepoPath.value || null,
				});
				if (list) {
					articles.value = list;
					repoInfo.value.article_count = list.length;
				}
			} catch (err) {
				console.error('Failed to list articles:', err);
			}
		};

		const loadGitStatus = async () => {
			try {
				const status = await invokeTauri('git_status', {
					customPath: customRepoPath.value || null,
				});
				gitStatusOutput.value = status || '';
			} catch (err) {
				gitStatusOutput.value = `获取状态出错: ${err}`;
			}
		};

		const hasGitChanges = computed(() => {
			return gitStatusOutput.value && gitStatusOutput.value.trim().length > 0;
		});

		const filteredArticles = computed(() => {
			if (!librarySearch.value.trim()) return articles.value;
			const q = librarySearch.value.toLowerCase().trim();
			return articles.value.filter(
				(a) =>
					a.title.toLowerCase().includes(q) ||
					a.abstract_text.toLowerCase().includes(q) ||
					a.filename.toLowerCase().includes(q)
			);
		});

		const renderedMarkdown = computed(() => {
			try {
				return marked(articleForm.value.content || '');
			} catch (e) {
				return articleForm.value.content;
			}
		});

		const autoGenerateFilename = () => {
			if (!articleForm.value.filename || articleForm.value.filename.startsWith('untitled')) {
				const safe = articleForm.value.title
					.toLowerCase()
					.replace(/[\s\p{P}]+/gu, '-')
					.replace(/^-+|-+$/g, '');
				articleForm.value.filename = safe ? `${safe}.md` : 'new-article.md';
			}
		};

		const getTabTitle = () => {
			if (currentTab.value === 'editor') return '文章创作工作台';
			if (currentTab.value === 'library') return '文章库与归档管理';
			if (currentTab.value === 'publish') return 'Git 发布与部署中心';
			if (currentTab.value === 'settings') return '工作区与偏好设置';
			return '';
		};

		const getCategoryName = (cat) => {
			switch (cat) {
				case 0: return 'TECH';
				case 1: return 'STORIES';
				case 2: return 'PRESS RELEASE';
				case 3: return 'CREATIVE';
				default: return 'UPDATE';
			}
		};

		const getCoverSrc = (pic) => {
			if (!pic || pic === 'none') return '';
			if (pic.startsWith('http') || pic.startsWith('data:')) return pic;
			return `${devServerUrl.value}/articles/${pic}`;
		};

		const handleImageUpload = (e) => {
			const file = e.target.files[0];
			if (!file) return;

			const reader = new FileReader();
			reader.onload = async (event) => {
				const base64 = event.target.result;
				const safeName = `img_${Date.now()}_${file.name.replace(/[^a-zA-Z0-9._-]/g, '_')}`;

				try {
					const savedName = await invokeTauri('save_image_base64', {
						customPath: customRepoPath.value || null,
						filename: safeName,
						base64Data: base64,
					});
					articleForm.value.picture = savedName || safeName;
					alert(`图片上传成功: ${savedName}`);
				} catch (err) {
					alert(`上传图片失败: ${err}`);
				}
			};
			reader.readAsDataURL(file);
		};

		const handleSaveArticle = async (andPush = false) => {
			if (!articleForm.value.title.trim()) {
				alert('请先输入文章标题');
				return;
			}

			if (!articleForm.value.filename.trim()) {
				autoGenerateFilename();
			}

			try {
				const savedFile = await invokeTauri('save_article', {
					customPath: customRepoPath.value || null,
					filename: articleForm.value.filename,
					title: articleForm.value.title,
					category: articleForm.value.category,
					abstractText: articleForm.value.abstract_text,
					picture: articleForm.value.picture,
					aiGenerated: articleForm.value.ai_generated,
					date: articleForm.value.date,
					content: articleForm.value.content,
				});

				await loadArticlesList();
				await loadGitStatus();

				if (andPush) {
					commitMessage.value = `publish: ${articleForm.value.title}`;
					currentTab.value = 'publish';
					await handleGitDeploy();
				} else {
					alert(`文章已成功保存至本地: ${savedFile}`);
				}
			} catch (err) {
				alert(`保存失败: ${err}`);
			}
		};

		const loadArticleToEditor = async (filename) => {
			try {
				const data = await invokeTauri('get_article', {
					customPath: customRepoPath.value || null,
					filename,
				});
				if (data) {
					articleForm.value = {
						filename: data.meta.filename,
						title: data.meta.title,
						category: data.meta.category,
						abstract_text: data.meta.abstract_text,
						picture: data.meta.picture,
						ai_generated: data.meta.ai_generated,
						date: data.meta.date,
						content: data.content,
					};
					currentTab.value = 'editor';
				}
			} catch (err) {
				alert(`加载文章失败: ${err}`);
			}
		};

		const handleDeleteArticle = async (filename) => {
			if (confirm(`确定要永久删除文章 "${filename}" 吗？此操作无法撤销。`)) {
				try {
					await invokeTauri('delete_article', {
						customPath: customRepoPath.value || null,
						filename,
					});
					await loadArticlesList();
					alert('删除成功');
				} catch (err) {
					alert(`删除失败: ${err}`);
				}
			}
		};

		const handleGitDeploy = async () => {
			isPublishing.value = true;
			gitLogOutput.value = '正在执行 git add, commit, push origin main...\n';

			try {
				const res = await invokeTauri('git_publish', {
					customPath: customRepoPath.value || null,
					commitMsg: commitMessage.value,
				});

				isPublishing.value = false;
				if (res) {
					gitSuccess.value = res.success;
					gitLogOutput.value = `${res.output}\n${res.error}`;
					await loadGitStatus();
				}
			} catch (err) {
				isPublishing.value = false;
				gitSuccess.value = false;
				gitLogOutput.value = `Git 操作执行异常: ${err}`;
			}
		};

		const openBrowser = (url) => {
			invokeTauri('open_external_url', { url });
		};

		const openArticleInWeb = (filename) => {
			openBrowser(`${devServerUrl.value}/article/${filename}`);
		};

		const resetEditor = () => {
			articleForm.value = {
				filename: '',
				title: '',
				category: 0,
				abstract_text: '',
				picture: 'none',
				ai_generated: false,
				date: '2026年10月25日',
				content: '# 新文章\n\n开始撰写内容...',
			};
		};

		const insertText = (prefix, suffix = '') => {
			const ta = markdownTextarea.value;
			if (!ta) return;
			const start = ta.selectionStart;
			const end = ta.selectionEnd;
			const text = articleForm.value.content;
			const selected = text.substring(start, end);

			articleForm.value.content =
				text.substring(0, start) + prefix + selected + suffix + text.substring(end);

			setTimeout(() => {
				ta.focus();
				ta.setSelectionRange(start + prefix.length, end + prefix.length);
			}, 0);
		};

		const insertTab = () => {
			insertText('  ');
		};

		const refreshRepo = async () => {
			await loadRepoInfo();
			await loadArticlesList();
			await loadGitStatus();
			alert('工作区设置已更新并重新加载。');
		};

		onMounted(() => {
			loadRepoInfo();
			loadArticlesList();
			loadGitStatus();
		});

		return {
			currentTab,
			editorView,
			repoInfo,
			articles,
			librarySearch,
			customRepoPath,
			devServerUrl,
			commitMessage,
			gitStatusOutput,
			gitLogOutput,
			gitSuccess,
			isPublishing,
			hasGitChanges,
			articleForm,
			markdownTextarea,
			filteredArticles,
			renderedMarkdown,
			getTabTitle,
			getCategoryName,
			getCoverSrc,
			autoGenerateFilename,
			handleImageUpload,
			handleSaveArticle,
			loadArticleToEditor,
			handleDeleteArticle,
			handleGitDeploy,
			openBrowser,
			openArticleInWeb,
			resetEditor,
			insertText,
			insertTab,
			refreshRepo,
			loadArticlesList,
			loadGitStatus,
		};
	},
};
</script>

<style>
/* Apple macOS Sequoia Window System */
:root {
  --apple-bg: #fbfbfd;
  --apple-surface: #ffffff;
  --apple-sidebar: #f5f5f7;
  --apple-border: #e5e5ea;
  --apple-accent: #0071e3;
  --apple-accent-hover: #0077ed;
  --apple-text-primary: #1d1d1f;
  --apple-text-secondary: #86868b;
  --apple-radius-sm: 8px;
  --apple-radius-md: 12px;
  --apple-radius-lg: 18px;
  --apple-radius-pill: 980px;
  --apple-font: -apple-system, BlinkMacSystemFont, "SF Pro Display", "SF Pro Text", system-ui, sans-serif;
  --apple-mono: "SF Mono", Menlo, Consolas, Monaco, monospace;
}

*, *::before, *::after {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  font-family: var(--apple-font);
  background-color: var(--apple-bg);
  color: var(--apple-text-primary);
  user-select: none;
  overflow: hidden;
  height: 100vh;
}

/* Main Window Layout */
.macos-app-window {
  display: flex;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background: var(--apple-bg);
}

/* Sidebar */
.macos-sidebar {
  width: 250px;
  background: var(--apple-sidebar);
  border-right: 1px solid var(--apple-border);
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 20px 16px;
  flex-shrink: 0;
}

.sidebar-header {
  margin-bottom: 24px;
}

.app-brand-pill {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: #ffffff;
  border-radius: var(--apple-radius-pill);
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
}

.apple-logo-svg {
  fill: var(--apple-text-primary);
}

.app-brand-name {
  font-size: 13px;
  font-weight: 600;
  letter-spacing: -0.01em;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 6px;
  flex: 1;
}

.nav-btn {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  border-radius: var(--apple-radius-md);
  border: none;
  background: transparent;
  color: #48484a;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  position: relative;
}

.nav-btn:hover {
  background: rgba(0, 0, 0, 0.05);
  color: var(--apple-text-primary);
}

.nav-btn.active {
  background: var(--apple-accent);
  color: #ffffff;
}

.nav-count-badge {
  margin-left: auto;
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 980px;
  background: rgba(0, 0, 0, 0.08);
}

.nav-btn.active .nav-count-badge {
  background: rgba(255, 255, 255, 0.25);
  color: #ffffff;
}

.git-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #ff9500;
  margin-left: auto;
}

.sidebar-footer {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding-top: 16px;
  border-top: 1px solid var(--apple-border);
}

.repo-meta-box {
  font-size: 11px;
  color: var(--apple-text-secondary);
  line-height: 1.5;
}

.preview-web-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: var(--apple-radius-pill);
  border: 1px solid var(--apple-border);
  background: #ffffff;
  color: var(--apple-text-primary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.preview-web-btn:hover {
  background: #f0f0f2;
}

/* Workspace */
.macos-workspace {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Toolbar */
.workspace-toolbar {
  height: 56px;
  border-bottom: 1px solid var(--apple-border);
  background: #ffffff;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
}

.toolbar-title {
  font-size: 16px;
  font-weight: 700;
  margin: 0;
  color: var(--apple-text-primary);
}

.segmented-control {
  display: flex;
  background: #f0f0f2;
  padding: 3px;
  border-radius: var(--apple-radius-sm);
}

.segment-btn {
  border: none;
  background: transparent;
  padding: 5px 14px;
  font-size: 12px;
  font-weight: 500;
  border-radius: 6px;
  cursor: pointer;
  color: var(--apple-text-secondary);
  transition: all 0.15s ease;
}

.segment-btn.active {
  background: #ffffff;
  color: var(--apple-text-primary);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.apple-btn-secondary {
  padding: 6px 14px;
  border-radius: var(--apple-radius-pill);
  border: 1px solid var(--apple-border);
  background: #ffffff;
  color: var(--apple-text-primary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.apple-btn-secondary:hover {
  background: #f5f5f7;
}

.apple-btn-primary {
  padding: 6px 16px;
  border-radius: var(--apple-radius-pill);
  border: none;
  background: var(--apple-accent);
  color: #ffffff;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.apple-btn-primary:hover {
  background: var(--apple-accent-hover);
}

/* Tab Panes */
.tab-pane {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

/* Meta Card */
.meta-card {
  background: #ffffff;
  border: 1px solid var(--apple-border);
  border-radius: var(--apple-radius-lg);
  padding: 24px;
  margin-bottom: 20px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.03);
}

.title-input {
  width: 100%;
  border: none;
  font-size: 24px;
  font-weight: 700;
  outline: none;
  color: var(--apple-text-primary);
  padding: 4px 0 16px;
  border-bottom: 1px solid var(--apple-border);
  margin-bottom: 18px;
}

.title-input::placeholder {
  color: #c7c7cc;
}

.meta-grid {
  display: grid;
  grid-template-columns: 1.5fr 1.2fr 1fr 0.8fr;
  gap: 16px;
  align-items: center;
}

.field-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--apple-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.category-segmented {
  display: flex;
  background: #f0f0f2;
  padding: 3px;
  border-radius: var(--apple-radius-sm);
}

.cat-segment-btn {
  flex: 1;
  border: none;
  background: transparent;
  padding: 6px 8px;
  font-size: 12px;
  font-weight: 500;
  border-radius: 6px;
  cursor: pointer;
  color: var(--apple-text-secondary);
}

.cat-segment-btn.active {
  background: #ffffff;
  color: var(--apple-text-primary);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.apple-text-input {
  padding: 8px 12px;
  border: 1px solid var(--apple-border);
  border-radius: var(--apple-radius-sm);
  font-size: 13px;
  outline: none;
  background: #ffffff;
}

.apple-text-input:focus {
  border-color: var(--apple-accent);
  box-shadow: 0 0 0 3px rgba(0, 113, 227, 0.12);
}

.apple-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--apple-border);
  border-radius: var(--apple-radius-sm);
  font-size: 13px;
  outline: none;
  resize: vertical;
  line-height: 1.5;
}

.apple-textarea:focus {
  border-color: var(--apple-accent);
  box-shadow: 0 0 0 3px rgba(0, 113, 227, 0.12);
}

/* Switch */
.switch-group {
  align-items: flex-start;
}

.apple-switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
}

.apple-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.switch-slider {
  position: absolute;
  cursor: pointer;
  top: 0; left: 0; right: 0; bottom: 0;
  background-color: #e5e5ea;
  transition: .3s;
  border-radius: 980px;
}

.switch-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: .3s;
  border-radius: 50%;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

input:checked + .switch-slider {
  background-color: var(--apple-accent);
}

input:checked + .switch-slider:before {
  transform: translateX(20px);
}

.mt-12 { margin-top: 14px; }
.flex-1 { flex: 1; }

.cover-picker-row {
  display: flex;
  gap: 10px;
}

.upload-btn-label {
  padding: 8px 16px;
  border: 1px solid var(--apple-border);
  border-radius: var(--apple-radius-sm);
  background: #f5f5f7;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  white-space: nowrap;
}

.upload-btn-label:hover {
  background: #e8e8ed;
}

.hidden-file-input {
  display: none;
}

.cover-preview-box {
  margin-top: 10px;
  position: relative;
  display: inline-block;
}

.cover-preview-img {
  max-height: 140px;
  border-radius: var(--apple-radius-md);
  border: 1px solid var(--apple-border);
}

.remove-cover-btn {
  position: absolute;
  top: 6px;
  right: 6px;
  background: rgba(0, 0, 0, 0.6);
  color: #ffffff;
  border: none;
  font-size: 11px;
  padding: 4px 8px;
  border-radius: 980px;
  cursor: pointer;
}

/* Editor Container */
.editor-container {
  display: grid;
  height: calc(100vh - 350px);
  min-height: 400px;
  gap: 20px;
}

.editor-container.split {
  grid-template-columns: 1fr 1fr;
}

.editor-container.edit {
  grid-template-columns: 1fr;
}

.editor-container.preview {
  grid-template-columns: 1fr;
}

/* Pane */
.editor-pane, .preview-pane {
  background: #ffffff;
  border: 1px solid var(--apple-border);
  border-radius: var(--apple-radius-lg);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.md-toolbar {
  display: flex;
  gap: 4px;
  padding: 8px 12px;
  background: #fbfbfd;
  border-bottom: 1px solid var(--apple-border);
}

.tool-btn {
  border: none;
  background: transparent;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  color: var(--apple-text-secondary);
}

.tool-btn:hover {
  background: #e8e8ed;
  color: var(--apple-text-primary);
}

.code-editor-textarea {
  flex: 1;
  width: 100%;
  border: none;
  outline: none;
  padding: 16px 20px;
  font-family: var(--apple-mono);
  font-size: 14px;
  line-height: 1.7;
  color: var(--apple-text-primary);
  resize: none;
  user-select: text;
}

/* Live Apple Preview */
.preview-pane {
  overflow-y: auto;
  padding: 32px 36px;
  user-select: text;
}

.apple-preview-wrapper {
  max-width: 680px;
  margin: 0 auto;
}

.preview-header-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 14px;
}

.preview-badge {
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.06em;
  color: var(--apple-text-secondary);
  text-transform: uppercase;
}

.preview-dot {
  color: #c7c7cc;
}

.preview-date {
  font-size: 12px;
  color: var(--apple-text-secondary);
}

.preview-ai-pill {
  font-size: 10px;
  font-weight: 600;
  color: var(--apple-accent);
  background: #e8f2fd;
  padding: 2px 8px;
  border-radius: 980px;
}

.preview-article-title {
  font-size: 32px;
  font-weight: 700;
  line-height: 1.2;
  margin: 0 0 16px;
  letter-spacing: -0.02em;
}

.preview-meta-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--apple-text-secondary);
  padding-bottom: 20px;
  border-bottom: 1px solid var(--apple-border);
  margin-bottom: 24px;
}

.preview-cover-banner {
  margin-bottom: 24px;
  border-radius: var(--apple-radius-md);
  overflow: hidden;
}

.banner-img {
  width: 100%;
  max-height: 320px;
  object-fit: cover;
  display: block;
}

.preview-html-body {
  font-size: 16px;
  line-height: 1.75;
  color: #333336;
}

.preview-html-body h2 {
  font-size: 24px;
  margin: 32px 0 14px;
}

.preview-html-body h3 {
  font-size: 20px;
  margin: 24px 0 12px;
}

.preview-html-body p {
  margin: 0 0 18px;
}

.preview-html-body blockquote {
  margin: 20px 0;
  padding: 12px 20px;
  background: #f5f5f7;
  border-left: 4px solid var(--apple-accent);
  border-radius: 0 12px 12px 0;
  color: var(--apple-text-secondary);
}

.preview-html-body pre {
  background: #1e1e1e;
  color: #d4d4d4;
  padding: 16px;
  border-radius: 12px;
  overflow-x: auto;
  font-family: var(--apple-mono);
  font-size: 13px;
}

/* Library View */
.library-search-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.library-search-input {
  width: 380px;
  padding: 10px 16px;
  border-radius: var(--apple-radius-pill);
  border: 1px solid var(--apple-border);
  background: #ffffff;
  outline: none;
  font-size: 14px;
}

.library-count-tag {
  font-size: 13px;
  color: var(--apple-text-secondary);
}

.articles-table-wrapper {
  background: #ffffff;
  border: 1px solid var(--apple-border);
  border-radius: var(--apple-radius-lg);
  overflow: hidden;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.03);
}

.apple-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
  text-align: left;
}

.apple-table th {
  padding: 14px 18px;
  background: #fbfbfd;
  border-bottom: 1px solid var(--apple-border);
  color: var(--apple-text-secondary);
  font-weight: 600;
  font-size: 12px;
}

.apple-table td {
  padding: 14px 18px;
  border-bottom: 1px solid var(--apple-border);
  vertical-align: middle;
}

.apple-table tr:last-child td {
  border-bottom: none;
}

.thumb-cell {
  width: 60px;
}

.table-thumb {
  width: 50px;
  height: 40px;
  border-radius: 6px;
  object-fit: cover;
  display: block;
}

.table-thumb-empty {
  width: 50px;
  height: 40px;
  border-radius: 6px;
  background: #f0f0f2;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  color: #8e8e93;
}

.title-cell {
  max-width: 260px;
}

.table-article-title {
  font-weight: 600;
  color: var(--apple-text-primary);
  margin-bottom: 2px;
}

.table-article-abstract {
  font-size: 12px;
  color: var(--apple-text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.table-cat-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 980px;
  background: #f0f0f2;
  font-size: 11px;
  font-weight: 600;
}

.filename-cell {
  font-family: var(--apple-mono);
  color: var(--apple-text-secondary);
  font-size: 12px;
}

.text-right {
  text-align: right;
}

.tbl-action-btn {
  padding: 4px 10px;
  border-radius: 6px;
  border: 1px solid var(--apple-border);
  background: #ffffff;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  margin-left: 6px;
}

.tbl-action-btn.primary {
  background: var(--apple-accent);
  color: #ffffff;
  border-color: var(--apple-accent);
}

.tbl-action-btn.danger {
  color: #ff3b30;
}

/* Publish View */
.publish-card, .settings-card {
  background: #ffffff;
  border: 1px solid var(--apple-border);
  border-radius: var(--apple-radius-lg);
  padding: 36px;
  max-width: 760px;
  margin: 0 auto;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.04);
}

.publish-title, .settings-title {
  font-size: 24px;
  font-weight: 700;
  margin: 0 0 10px;
}

.publish-desc {
  font-size: 14px;
  color: var(--apple-text-secondary);
  margin: 0 0 24px;
  line-height: 1.5;
}

.commit-input-box {
  margin-bottom: 24px;
}

.git-status-preview {
  margin-bottom: 24px;
}

.status-preview-header {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  font-weight: 600;
  color: var(--apple-text-secondary);
  margin-bottom: 8px;
}

.refresh-status-btn {
  background: none;
  border: none;
  color: var(--apple-accent);
  cursor: pointer;
  font-size: 12px;
}

.git-status-output {
  background: #f5f5f7;
  border: 1px solid var(--apple-border);
  border-radius: var(--apple-radius-sm);
  padding: 12px 16px;
  font-family: var(--apple-mono);
  font-size: 12px;
  margin: 0;
  max-height: 160px;
  overflow-y: auto;
}

.publish-action-row {
  display: flex;
  justify-content: center;
  margin: 32px 0 20px;
}

.publish-big-btn {
  padding: 12px 36px;
  font-size: 15px;
  border-radius: 980px;
}

.git-log-console {
  margin-top: 24px;
  background: #1e1e1e;
  border-radius: var(--apple-radius-md);
  padding: 16px 20px;
  color: #ffffff;
}

.console-header {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  margin-bottom: 10px;
  padding-bottom: 8px;
  border-bottom: 1px solid #333333;
}

.status-ok { color: #34c759; font-weight: 600; }
.status-err { color: #ff3b30; font-weight: 600; }

.console-body {
  font-family: var(--apple-mono);
  font-size: 12px;
  line-height: 1.6;
  margin: 0;
  max-height: 240px;
  overflow-y: auto;
  color: #cccccc;
}

/* Settings View */
.settings-item {
  margin-bottom: 24px;
}

.settings-hint {
  font-size: 12px;
  color: var(--apple-text-secondary);
  margin: 6px 0 0;
}

.settings-actions {
  display: flex;
  justify-content: flex-end;
}
</style>
