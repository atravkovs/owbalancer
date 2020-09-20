<template>
  <Modal
    title="Balance"
    :isActive="isActive"
    customAction="Balance"
    @close-modal="closeModal"
    @save-changes="balance"
  >
    <div class="mb-3">
      <label class="form-label">Balance Type</label>
      <br />
      <div class="btn-group">
        <input type="radio" value="full" id="balance1" class="btn-check" v-model="balanceType" />
        <label class="btn btn-primary" for="balance1">Full</label>
        <input type="radio" value="half" id="balance2" class="btn-check" v-model="balanceType" />
        <label class="btn btn-primary" for="balance2">Half</label>
        <input type="radio" value="final" id="balance3" class="btn-check" v-model="balanceType" />
        <label class="btn btn-primary" for="balance3">Final</label>
      </div>
    </div>
    <div class="mb-3">
      <label for="dispersionRange" class="form-label">
        Dispersion:
        <b>±{{ range }}</b>
      </label>
      <input
        id="dispersionRange"
        type="range"
        class="form-range"
        min="0"
        max="60"
        step="1"
        v-model="range"
      />
    </div>
  </Modal>
</template>

<script lang="ts">
/* eslint-disable */
import { computed, defineComponent, ref } from 'vue';

import Modal from '@/components/Helpers/Modal.vue';
import MutationTypes from '@/store/mutation-types';
import { useStore } from '@/store';

const wasm = import('@/../pkg/index.js');

export default defineComponent({
  name: 'Balance',
  components: { Modal },
  setup() {
    const range = ref(30);
    const balanceType = ref('full');

    const store = useStore();
    const isActive = computed(() => store.state.isBalance);
    const reservedPlayers = computed(() => store.state.reservedPlayers);
    const stateTeams = computed(() => store.state.teams);

    const closeModal = () => {
      store.commit(MutationTypes.TOGGLE_BALANCE);
    };

    const fullBalance: (lib: any) => any = (lib) => {
      return lib.balance(store.state.players, +range.value);
    };

    const halfBalance: (lib: any) => any = (lib) => {
      return lib.balance_half(store.state.players, +range.value);
    };

    const finalBalance: (lib: any, data: any) => any = (
      lib,
      { teamsCopy, reserveCopy }
    ) => {
      return lib.balance_final(
        store.state.players,
        +range.value,
        reserveCopy,
        teamsCopy
      );
    };

    const conditionalBalance: (lib: any, data: any) => any = (lib, data) => {
      if (balanceType.value === 'half') {
        return halfBalance(lib);
      }

      if (balanceType.value === 'final') {
        return finalBalance(lib, data);
      }

      return fullBalance(lib);
    };

    const balance = () => {
      const teamsCopy = [...stateTeams.value];
      const reserveCopy = [...reservedPlayers.value];

      store.commit(MutationTypes.CLEAR_TEAMS);

      wasm
        .then((lib) => {
          try {
            const { leftovers, teams } = conditionalBalance(lib, {
              teamsCopy,
              reserveCopy,
            });

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

    return {
      closeModal,
      isActive,
      range,
      balance,
      balanceType,
    };
  },
});
</script>

<style lang="scss" scoped>
.form-range {
  &::-webkit-slider-runnable-track {
    background: linear-gradient(to right, #33ccff 0%, #ff0000 100%);
  }
}
</style>
