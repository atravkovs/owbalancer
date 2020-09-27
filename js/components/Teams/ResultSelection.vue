<template>
  <modal
    title="Select Result"
    variant="large"
    :isActive="isActive"
    :hideAction="true"
    @close-modal="closeModal"
  >
    <div class="ResultSelection">
      <result v-for="(balance, i) in results" :key="i" :balance="balance" />
    </div>
  </modal>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

import Modal from '@/components/Helpers/Modal.vue';
import Result from '@/components/Teams/Result.vue';

export default defineComponent({
  name: 'ResultSelection',
  components: { Modal, Result },
  setup() {
    const store = useStore();
    const isActive = computed(() => store.state.isSelection);
    const results = computed(() => store.state.balancerResults);

    const closeModal = () => {
      store.commit(MutationTypes.TOGGLE_SELECTION);
    };

    return { isActive, closeModal, results };
  },
});
</script>

<style lang="scss" scoped>
.ResultSelection {
  display: grid;
  grid-template-rows: auto;
  grid-template-columns: repeat(auto-fit, 200px);
  column-gap: 1rem;
  row-gap: 1rem;
}
</style>
