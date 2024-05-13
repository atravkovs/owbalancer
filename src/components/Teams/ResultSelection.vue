<template>
  <modal
    title="Select Result"
    variant="large"
    :isActive="isActive"
    :hideAction="true"
    fullscreen="md-down"
    @close-modal="closeModal"
  >
    <div v-if="results.length > 0" class="ResultSelection">
      <result
        v-for="(balance, i) in results"
        :key="i"
        :balance="balance"
        :is-selected="isActiveBalance(balance)"
      />
    </div>
    <div v-else>
      Were unable to balance
    </div>
  </modal>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

import Modal from '@/components/Helpers/Modal.vue';
import Result from '@/components/Teams/Result.vue';
import { Balance } from '@/objects/balance';

export default defineComponent({
  name: 'ResultSelection',
  components: { Modal, Result },
  setup() {
    const store = useStore();
    const isActive = computed(() => store.state.isSelection);
    const results = computed(() =>
      store.state.balancerResults
        .filter(v => v.leftovers.length === 0)
        .sort((a, b) => a.dispersion - b.dispersion)
        .concat(
          store.state.balancerResults
            .filter(v => v.leftovers.length !== 0)
            .sort((a, b) => a.dispersion - b.dispersion)
        )
    );

    const closeModal = () => {
      store.commit(MutationTypes.TOGGLE_SELECTION, undefined);
    };

    const storeTeams = computed(() => store.state.teams);
    const isActiveBalance = (balance: Balance) => {
      return (
        storeTeams.value.map(t => t.uuid).join('|') === balance.teams.map(t => t.uuid).join('|')
      );
    };

    return { isActive, closeModal, results, isActiveBalance };
  },
});
</script>

<style lang="scss" scoped>
.ResultSelection {
  display: grid;
  grid-template-rows: auto;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  column-gap: 1rem;
  row-gap: 1rem;
}
</style>
