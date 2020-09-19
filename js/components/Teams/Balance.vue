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
        max="150"
        step="5"
        v-model="range"
      />
    </div>
  </Modal>
</template>

<script lang="ts">
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

    const closeModal = () => {
      store.commit(MutationTypes.TOGGLE_BALANCE);
    };

    const fullBalance = () => {
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

    const halfBalance = () => {
      store.commit(MutationTypes.CLEAR_TEAMS);

      wasm
        .then((lib) => {
          try {
            const { leftovers, teams } = lib.balance_half(
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

    const balance = () => {
      if (balanceType.value === 'full') {
        fullBalance();
      } else if (balanceType.value === 'half') {
        halfBalance();
      }
    };

    return { closeModal, isActive, range, balance, balanceType };
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
