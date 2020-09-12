<template>
  <h3>Teams</h3>
  <div class="d-flex">
    <button class="btn btn-sm btn-primary" @click="balance">Balance</button>
    <button class="btn btn-sm btn-danger ml-2" @click="clear">Clear</button>
    <span>Value: {{ range }}</span>
    <input
      type="range"
      style="width:250px;"
      class="form-range"
      min="0"
      max="50"
      step="1"
      v-model="range"
    />
  </div>
  <div class="teams pb-5 mb-5">
    <team v-for="team in teams" :key="team.name" :team="team" />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from 'vue';
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

import Team from '@/components/Teams/Team.vue';

const wasm = import('@/../pkg/index.js');

export default defineComponent({
  name: 'Teams',
  components: { Team },
  setup() {
    const store = useStore();
    const storeTeams = computed(() => store.state.teams);
    const range = ref(0);

    const balance = () => {
      wasm
        .then((lib) => {
          const { leftovers, teams } = lib.balance(
            store.state.players,
            +range.value
          );
          const ignoredUuids = leftovers.reduce((acc, leftover) => {
            acc.push(leftover.uuid);
            return acc;
          }, []);

          store.commit(MutationTypes.RESERVE_PLAYERS, ignoredUuids);
          store.commit(MutationTypes.ADD_TEAMS, teams);

          console.log('Received: ', leftovers, teams);
        })
        .catch(console.error);
    };

    const clear = () => {
      store.commit(MutationTypes.CLEAR_TEAMS);
    };

    return { balance, range, clear, teams: storeTeams };
  },
});
</script>

<style lang="scss" scoped>
.teams {
  display: grid;
  grid-template-rows: auto;
  grid-template-columns: auto auto auto;
  column-gap: 2rem;
  row-gap: 2rem;
  margin-top: 2rem;
  max-width: 1024px;
}
</style>
