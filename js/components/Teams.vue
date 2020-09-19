<template>
  <h3>Teams</h3>
  <div class="d-flex">
    <button class="btn btn-sm btn-secondary" @click="addNew">New</button>
    <button class="btn btn-sm btn-primary ml-2" @click="balance">Balance</button>
    <button class="btn btn-sm btn-danger ml-2" @click="clear">Clear</button>
    <span>Dispersion value: {{ range }}</span>
    <input
      type="range"
      style="width:250px;"
      class="form-range"
      min="0"
      max="150"
      step="5"
      v-model="range"
    />
  </div>
  <div v-if="teams.length > 0">
    <span>Min SR: {{ minSr }}</span>
    <span class="ml-2">Max SR: {{ maxSr }}</span>
    <span class="ml-2">Difference: {{ maxSr - minSr }}</span>
    <span class="ml-2">Average: {{ avgSr }}</span>
  </div>
  <div class="teams pb-5 mb-5 overflow-auto h-80vh">
    <team v-for="team in teams" :key="team.uuid" :team="team" />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from 'vue';
import { useStore } from '@/store';
import TObj from '@/objects/team';
import MutationTypes from '@/store/mutation-types';

import Team from '@/components/Teams/Team.vue';

const wasm = import('@/../pkg/index.js');

export default defineComponent({
  name: 'Teams',
  components: { Team },
  setup() {
    const store = useStore();
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
    const range = ref(0);

    const balance = () => {
      store.commit(MutationTypes.CLEAR_TEAMS);

      wasm
        .then((lib) => {
          try {
            const { leftovers, teams } = lib.balance(
              store.state.players,
              +range.value
            );
            const ignoredUuids = leftovers.reduce((acc, leftover) => {
              acc.push(leftover.uuid);
              return acc;
            }, []);

            console.log('Teams', teams);
            store.commit(MutationTypes.RESERVE_PLAYERS, ignoredUuids);
            store.commit(MutationTypes.ADD_TEAMS, teams);
          } catch (e) {
            console.error(e.message);
          }
        })
        .catch((e) => {
          console.error(e.message);
        });
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

    return {
      range,
      clear,
      maxSr,
      minSr,
      avgSr,
      addNew,
      balance,
      teams: storeTeams,
    };
  },
});
</script>

<style lang="scss" scoped>
.teams {
  display: grid;
  grid-template-rows: auto;
  grid-template-columns: repeat(auto-fit, 250px);
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
