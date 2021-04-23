<template>
  <img class="s-2em" v-if="rank <= 199" src="@/assets/divisions/div20.png" />
  <img class="s-2em" v-else-if="rank <= 299" src="@/assets/divisions/div19.png" />
  <img class="s-2em" v-else-if="rank <= 399" src="@/assets/divisions/div18.png" />
  <img class="s-2em" v-else-if="rank <= 499" src="@/assets/divisions/div17.png" />
  <img class="s-2em" v-else-if="rank <= 599" src="@/assets/divisions/div16.png" />
  <img class="s-2em" v-else-if="rank <= 699" src="@/assets/sdivisions/div15.png" />
  <img class="s-2em" v-else-if="rank <= 799" src="@/assets/sdivisions/div14.png" />
  <img class="s-2em" v-else-if="rank <= 899" src="@/assets/sdivisions/div13.png" />
  <img class="s-2em" v-else-if="rank <= 999" src="@/assets/sdivisions/div12.png" />
  <img class="s-2em" v-else-if="rank <= 1099" src="@/assets/sdivisions/div11.png" />
  <img class="s-2em" v-else-if="rank <= 1199" src="@/assets/sdivisions/div10.png" />
  <img class="s-2em" v-else-if="rank <= 1299" src="@/assets/sdivisions/div9.png" />
  <img class="s-2em" v-else-if="rank <= 1399" src="@/assets/sdivisions/div8.png" />
  <img class="s-2em" v-else-if="rank <= 1499" src="@/assets/sdivisions/div7.png" />
  <img class="s-2em" v-else-if="rank <= 1599" src="@/assets/sdivisions/div6.png" />
  <img class="s-2em" v-else-if="rank <= 1699" src="@/assets/sdivisions/div5.png" />
  <img class="s-2em" v-else-if="rank <= 1799" src="@/assets/sdivisions/div4.png" />
  <img class="s-2em" v-else-if="rank <= 1899" src="@/assets/sdivisions/div3.png" />
  <img class="s-2em" v-else-if="rank <= 1999" src="@/assets/sdivisions/div2.png" />
  <img class="s-2em" v-else-if="rank <= 2100" src="@/assets/sdivisions/div1.png" />
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
