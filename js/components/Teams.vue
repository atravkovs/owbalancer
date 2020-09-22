<template>
  <h3>Teams</h3>
  <div class="d-flex justify-content-between">
    <div class="d-flex">
      <button class="btn btn-sm btn-secondary" @click="addNew">New</button>
      <button class="btn btn-sm btn-primary ml-2" @click="balance">Balance</button>
      <button class="btn btn-sm btn-danger mx-2" @click="clear">Clear</button>
      <button class="btn btn-sm btn-secondary mx-2" @click="empty">Empty</button>
      <export-teams />
    </div>
    <div class="form-check">
      <input
        type="checkbox"
        id="toggleSRMode"
        class="form-check-input"
        :value="showBalancerSR"
        @change="toggleSR"
      />
      <label for="toggleSRMode" class="form-check-label">Show balancer SR</label>
    </div>
  </div>
  <div v-if="teams.length > 0">
    <span>Min SR: {{ minSr }}</span>
    <span class="ml-2">Max SR: {{ maxSr }}</span>
    <span class="ml-2">Difference: {{ maxSr - minSr }}</span>
    <span class="ml-2">Average: {{ avgSr }}</span>
  </div>
  <div class="teams overflow-auto h-80vh">
    <team v-for="team in teams" :key="team.uuid" :team="team" />
  </div>
  <balance />
</template>

<script lang="ts">
import { computed, defineComponent, ref } from 'vue';
import { useStore } from '@/store';
import TObj from '@/objects/team';
import MutationTypes from '@/store/mutation-types';

import Team from '@/components/Teams/Team.vue';
import Balance from '@/components/Teams/Balance.vue';
import ExportTeams from '@/components/Teams/ExportTeams.vue';

export default defineComponent({
  name: 'Teams',
  components: { Team, Balance, ExportTeams },
  setup() {
    const store = useStore();
    const showBalancerSR = ref(store.state.showBalancerSR);

    const storeTeams = computed(() => store.state.teams);
    const maxSr = computed(() =>
      Math.floor(
        store.state.teams.reduce(
          (acc, team) => (acc < team.avgSr ? team.avgSr : acc),
          0
        )
      )
    );
    const minSr = computed(() =>
      Math.floor(
        store.state.teams.reduce(
          (acc, team) => (acc > team.avgSr ? team.avgSr : acc),
          10000
        )
      )
    );
    const avgSr = computed(() =>
      Math.floor(
        store.state.teams.reduce((acc, team) => acc + team.avgSr, 0) /
          store.state.teams.length
      )
    );

    const balance = () => {
      store.commit(MutationTypes.TOGGLE_BALANCE);
    };

    const clear = () => {
      store.commit(MutationTypes.CLEAR_TEAMS);
    };

    const addNew = () => {
      const newTeam = TObj.createEmptyTeam();

      if (store.state.teams.length <= 0) {
        const playerIds = Object.keys(store.state.players);
        store.commit(MutationTypes.RESERVE_PLAYERS, playerIds);
      }

      store.commit(MutationTypes.ADD_TEAM, newTeam);
    };

    const empty = () => {
      store.commit(MutationTypes.EMPTY_TEAMS);
    };

    const toggleSR = () => {
      store.commit(MutationTypes.TOGGLE_BALANCER_SR);
    };

    return {
      empty,
      clear,
      maxSr,
      minSr,
      avgSr,
      addNew,
      balance,
      teams: storeTeams,
      toggleSR,
      showBalancerSR,
    };
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
