<template>
  <img class="s-2em" v-if="rank <= 100" src="@/assets/sdivisions/div15.png" />
  <img class="s-2em" v-else-if="rank <= 200" src="@/assets/sdivisions/div14.png" />
  <img class="s-2em" v-else-if="rank <= 300" src="@/assets/sdivisions/div13.png" />
  <img class="s-2em" v-else-if="rank <= 400" src="@/assets/sdivisions/div12.png" />
  <img class="s-2em" v-else-if="rank <= 500" src="@/assets/sdivisions/div11.png" />
  <img class="s-2em" v-else-if="rank <= 600" src="@/assets/sdivisions/div10.png" />
  <img class="s-2em" v-else-if="rank <= 700" src="@/assets/sdivisions/div9.png" />
  <img class="s-2em" v-else-if="rank <= 800" src="@/assets/sdivisions/div8.png" />
  <img class="s-2em" v-else-if="rank <= 900" src="@/assets/sdivisions/div7.png" />
  <img class="s-2em" v-else-if="rank <= 1000" src="@/assets/sdivisions/div6.png" />
  <img class="s-2em" v-else-if="rank <= 1100" src="@/assets/sdivisions/div5.png" />
  <img class="s-2em" v-else-if="rank <= 1200" src="@/assets/sdivisions/div4.png" />
  <img class="s-2em" v-else-if="rank <= 1300" src="@/assets/sdivisions/div3.png" />
  <img class="s-2em" v-else-if="rank <= 1400" src="@/assets/sdivisions/div2.png" />
  <img class="s-2em" v-else-if="rank <= 2000" src="@/assets/sdivisions/div1.png" />
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
