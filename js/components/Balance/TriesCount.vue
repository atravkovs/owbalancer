<template>
  <div class="mb-3">
    <label for="triesRange" class="form-label">
      Tries count:
      <b>{{ balancerOptions.triesCount }}</b>
    </label>
    <input
      id="triesRange"
      type="range"
      class="form-range"
      min="0"
      max="2500"
      step="1"
      v-model.number="balancerOptions.triesCount"
    />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from 'vue';

import { useStore } from '@/store';

export default defineComponent({
  name: 'TriesCount',
  setup() {
    const store = useStore();
    const storeBalancerOptions = computed(() => store.state.balancerOptions);
    const balancerOptions = ref(storeBalancerOptions);

    return { balancerOptions };
  },
});
</script>
