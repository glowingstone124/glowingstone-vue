<template>
  <div>
    <Nav/>
    <Title :msg="msg"/>
    <ArticleList :articles="articleList" />
    <NavBtn/>
    <Footer/>
  </div>
  <p class="counter">已经有{{ visitCount }}人访问了本站</p>
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
      visitCount: 114514,
      msg:"每月更新的可能性为lim -> 0，jvm痴子，全栈小白，性能优化魔怔人。"
    }
  },
  async mounted() {
    const currentDate = new Date();
    const Birthday = new Date("2025-04-28");
    if(currentDate === Birthday){
      this.msg = "祝我生日快乐";
      document.title = "Glowstone 萤石 | 祝我生日快乐"
    }
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
  },
  
}
</script>

<style>
@import "@/css/main.css";
p{
    padding: 10px;
    color: white;
    margin: auto;
}
.counter{
  text-align: center;
}
</style>
