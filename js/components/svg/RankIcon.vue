<template>
  <img v-if="rank <= 100" src="@/assets/divisions/div15.png" />
  <img v-else-if="rank <= 200" src="@/assets/divisions/div14.png" />
  <img v-else-if="rank <= 300" src="@/assets/divisions/div13.png" />
  <img v-else-if="rank <= 400" src="@/assets/divisions/div12.png" />
  <img v-else-if="rank <= 500" src="@/assets/divisions/div11.png" />
  <img v-else-if="rank <= 600" src="@/assets/divisions/div10.png" />
  <img v-else-if="rank <= 700" src="@/assets/divisions/div9.png" />
  <img v-else-if="rank <= 800" src="@/assets/divisions/div8.png" />
  <img v-else-if="rank <= 900" src="@/assets/divisions/div7.png" />
  <img v-else-if="rank <= 1000" src="@/assets/divisions/div6.png" />
  <img v-else-if="rank <= 1100" src="@/assets/divisions/div5.png" />
  <img v-else-if="rank <= 1200" src="@/assets/divisions/div4.png" />
  <img v-else-if="rank <= 1300" src="@/assets/divisions/div3.png" />
  <img v-else-if="rank <= 1400" src="@/assets/divisions/div2.png" />
  <img v-else-if="rank <= 2000" src="@/assets/divisions/div1.png" />
  <span v-else></span>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'RankIcon',
  props: {
    rank: Number,
  },
});
</script>
