<template>
  <Modal
    title="Balance"
    :isActive="isActive"
    customAction="Balance"
    @close-modal="closeModal"
    @save-changes="balance"
  >
    <div class="mb-3">
      <label for="dispersionRange" class="form-label">
        Dispersion value:
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
    const store = useStore();
    const range = ref(30);
    const isActive = computed(() => store.state.isBalance);

    const closeModal = () => {
      store.commit(MutationTypes.TOGGLE_BALANCE);
    };

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

    return { closeModal, isActive, range, balance };
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
