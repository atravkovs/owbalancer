<template>
  <img class="s-2em" v-if="rank <= 199" src="@/assets/sdivisions2/div20.png" />
  <img class="s-2em" v-else-if="rank <= 299" src="@/assets/sdivisions2/div19.png" />
  <img class="s-2em" v-else-if="rank <= 399" src="@/assets/sdivisions2/div18.png" />
  <img class="s-2em" v-else-if="rank <= 499" src="@/assets/sdivisions2/div17.png" />
  <img class="s-2em" v-else-if="rank <= 599" src="@/assets/sdivisions2/div16.png" />
  <img class="s-2em" v-else-if="rank <= 699" src="@/assets/sdivisions2/div15.png" />
  <img class="s-2em" v-else-if="rank <= 799" src="@/assets/sdivisions2/div14.png" />
  <img class="s-2em" v-else-if="rank <= 899" src="@/assets/sdivisions2/div13.png" />
  <img class="s-2em" v-else-if="rank <= 999" src="@/assets/sdivisions2/div12.png" />
  <img class="s-2em" v-else-if="rank <= 1099" src="@/assets/sdivisions2/div11.png" />
  <img class="s-2em" v-else-if="rank <= 1199" src="@/assets/sdivisions2/div10.png" />
  <img class="s-2em" v-else-if="rank <= 1299" src="@/assets/sdivisions2/div9.png" />
  <img class="s-2em" v-else-if="rank <= 1399" src="@/assets/sdivisions2/div8.png" />
  <img class="s-2em" v-else-if="rank <= 1499" src="@/assets/sdivisions2/div7.png" />
  <img class="s-2em" v-else-if="rank <= 1599" src="@/assets/sdivisions2/div6.png" />
  <img class="s-2em" v-else-if="rank <= 1699" src="@/assets/sdivisions2/div5.png" />
  <img class="s-2em" v-else-if="rank <= 1799" src="@/assets/sdivisions2/div4.png" />
  <img class="s-2em" v-else-if="rank <= 1899" src="@/assets/sdivisions2/div3.png" />
  <img class="s-2em" v-else-if="rank <= 1999" src="@/assets/sdivisions2/div2.png" />
  <img class="s-2em" v-else-if="rank <= 2100" src="@/assets/sdivisions2/div1.png" />
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
