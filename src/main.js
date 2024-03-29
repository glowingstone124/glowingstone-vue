import { createApp } from 'vue';
import App from './components/App.vue';
import Main from './App.vue'
import Friends from './components/Friends/Friends.vue';
import Working from './components/Working.vue';
import Article from './components/Article.vue';
import About from './components/About.vue';
import { createRouter, createWebHistory } from 'vue-router';

const app = createApp(Main);

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: App },
    { path: '/friends', component: Friends},
    { path: '/article/:articleId', component: Article},
    { path: '/archive', component: Working},
    { path: '/about', component: About}
  ],
});


app.use(router);
app.mount('#app');
