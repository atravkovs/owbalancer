<template>
  <div class="mb-3">
    <div class="form-check">
      <input type="checkbox" id="srScaling" class="form-check-input" v-model="adjustSr.isEnabled" />
      <label for="srScaling" class="form-check-label">Adjust player SR</label>
    </div>
    <div class="text-center mt-2">
      <adjust-role role="tank" />
      <adjust-role role="dps" />
      <adjust-role role="support" />
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from 'vue';

import { useStore } from '@/store';

import AdjustRole from './AdjustRole.vue';

export default defineComponent({
  name: 'AdjustRating',
  components: { AdjustRole },
  setup() {
    const store = useStore();
    const storeOptions = computed(() => store.state.balancerOptions.adjustSr);
    const adjustSr = ref(storeOptions);

    return { adjustSr };
  },
});
</script>

<style lang="scss">
.AdjustRole {
  width: min-content;
}
</style>
