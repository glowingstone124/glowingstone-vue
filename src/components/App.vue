<template>
  <div>
    <Nav/>
    <Title msg="High school student lives in Shanghai, individual Java & Web developer"/>
    <ArticleList :articles="articleList" />
    <NavBtn/>
    <Footer/>
  </div>
  <p>已经有{{ visitCount }}人访问了本站</p>
</template>

<script>
import Title from './Title.vue'
import ArticleList from './ArticleList.vue'
import NavBtn from './NavBtn.vue';
import Nav from './Nav.vue';
import Footer from './Footer.vue';

export default {
  name: 'App',
  components: {
    Title,
    ArticleList,
    NavBtn,
    Nav,
    Footer
  },
  data() {
    return {
      articleList: [],
      visitCount: 114514
    }
  },
  async mounted() {
    await this.incrementVisitCount();
  },
  methods: {
    async incrementVisitCount() {
  const response = await fetch("https://lenient-dove-46659.upstash.io/get/visitcount", {
    headers: {
      Authorization: "Bearer AbZDASQgYTZmZGM5MTctZjYyNi00YzdjLWE3ZWUtZWQyZjVhNzBhMzA0ZmNjZjMzZDRkMWFiNGYwYmIwMDQ2Yjg1NWNhMmJmYzU=" // 请替换为你的实际授权令牌
    }
  });
      const data = await response.json();
      const resultData = JSON.parse(data.result);
      var visitCount = resultData.value;

      console.log("Visit count:", visitCount);
      visitCount++;
      this.visitCount = visitCount;
      const updateResponse = await fetch("https://lenient-dove-46659.upstash.io/set/visitcount", {
        method: "POST",
        headers: {
          Authorization: "Bearer AbZDASQgYTZmZGM5MTctZjYyNi00YzdjLWE3ZWUtZWQyZjVhNzBhMzA0ZmNjZjMzZDRkMWFiNGYwYmIwMDQ2Yjg1NWNhMmJmYzU=", // 请替换为你的实际授权令牌
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          value: visitCount
        })
      });
      const updateData = await updateResponse.json();
      console.log("Visit count:", visitCount);
    }
  }
}
</script>

<style>
@import "@/css/main.css";
p{
    padding: 5vh;
    background-color: #384454;
    color: white;
}
</style>
