<template>
  <h3>Teams</h3>
  <div class="d-flex justify-content-between">
    <actions />
    <mode-toggler />
  </div>
  <stats v-if="teams.length > 0" />
  <div class="teams overflow-auto h-80vh">
    <team v-for="team in teams" :key="team.uuid" :team="team" />
  </div>
  <balance />
  <result-selection />
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';
import { useStore } from '@/store';

import Balance from '@/components/Balance.vue';
import Team from '@/components/Teams/Team.vue';
import Stats from '@/components/Teams/Stats.vue';
import Actions from '@/components/Teams/Actions.vue';
import ModeToggler from '@/components/Teams/ModeToggler.vue';
import ResultSelection from '@/components/Teams/ResultSelection.vue';

export default defineComponent({
  name: 'Teams',
  components: { Team, Balance, Stats, Actions, ModeToggler, ResultSelection },
  setup() {
    const store = useStore();
    const storeTeams = computed(() => store.state.teams);

    return { teams: storeTeams };
  },
});
</script>

<style lang="scss" scoped>
.teams {
  display: grid;
  grid-template-rows: auto;
  grid-template-columns: repeat(auto-fit, 275px);
  column-gap: 1rem;
  row-gap: 1rem;
  margin-top: 2rem;
  min-width: 700px;
}
.wf {
  width: 6rem;
}
.h-80vh {
  height: 80vh;
}
</style>
