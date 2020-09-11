<template>
  <img v-if="rank < 1500" class="s-2em" src="@/assets/rank_icons/Bronze.png" />
  <img v-else-if="rank < 2000" class="s-2em" src="@/assets/rank_icons/Silver.png" />
  <img v-else-if="rank < 2500" class="s-2em" src="@/assets/rank_icons/Gold.png" />
  <img v-else-if="rank < 3000" class="s-2em" src="@/assets/rank_icons/Platinum.png" />
  <img v-else-if="rank < 3500" class="s-2em" src="@/assets/rank_icons/Diamond.png" />
  <img v-else-if="rank < 4000" class="s-2em" src="@/assets/rank_icons/Master.png" />
  <img v-else-if="rank < 5000" class="s-2em" src="@/assets/rank_icons/Grandmaster.png" />
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
