<template>
  <h3>Teams</h3>
  <div>
    <button class="btn btn-sm btn-primary" @click="balance">Balance</button>
    <button class="btn btn-sm btn-danger ml-2">Add Team</button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

const wasm = import('@/../pkg/index.js');

export default defineComponent({
  name: 'Teams',
  setup() {
    const store = useStore();

    const balance = () => {
      wasm
        .then((lib) => {
          const { leftovers, teams } = lib.balance(store.state.players, 1000);
          const ignoredUuids = leftovers.reduce((acc, leftover) => {
            acc.push(leftover.uuid);
            return acc;
          }, []);

          store.commit(MutationTypes.RESERVE_PLAYERS, ignoredUuids);

          console.log('Received: ', leftovers, teams);
        })
        .catch(console.error);
    };

    return { balance };
  },
});
</script>
