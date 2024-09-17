import { createApp } from 'vue';
import App from './components/App.vue';
import Main from './App.vue'
import Friends from './components/Friends/Friends.vue';
import Article from './components/Article.vue';
import About from './components/About.vue';
import ArticleList from "@/components/ArticleList.vue";
import { createRouter, createWebHistory } from 'vue-router';

const app = createApp(Main);

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: App },
    { path: '/friends', component: Friends},
    { path: '/article/:articleId', component: Article},
    { path: '/archive', component: ArticleList},
    { path: '/about', component: About}
  ],
});


app.use(router);
app.mount('#app');
