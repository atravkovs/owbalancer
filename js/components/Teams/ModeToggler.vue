<template>
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
</template>

<script type="ts">
import { defineComponent, ref } from 'vue';

import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

export default defineComponent({
  name: 'ModeToggler',
  setup() {
    const store = useStore();
    const showBalancerSR = ref(store.state.showBalancerSR);

    const toggleSR = () => {
      store.commit(MutationTypes.TOGGLE_BALANCER_SR, undefined);
    };

    return { showBalancerSR, toggleSR };
  }
});
</script>
