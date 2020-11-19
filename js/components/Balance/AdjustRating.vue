<template>
  <div class="mb-3">
    <div class="form-check">
      <input type="checkbox" id="srScaling" class="form-check-input" v-model="adjustSr.isEnabled" />
      <label for="srScaling" class="form-check-label">Adjust player SR by main class</label>
    </div>
    <div class="d-flex justify-content-around text-center mt-2">
      <div>
        <div>
          <bezier v-model="adjustSr.tank" />
        </div>
        <span>Tank</span>
      </div>
      <div>
        <div>
          <bezier v-model="adjustSr.dps" />
        </div>
        <span>DPS</span>
      </div>
      <div>
        <div>
          <bezier v-model="adjustSr.support" />
        </div>
        <span>Support</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from 'vue';

import { useStore } from '@/store';

import Bezier from '@/components/Helpers/Bezier.vue';

export default defineComponent({
  name: 'AdjustSr',
  components: { Bezier },
  setup() {
    const store = useStore();
    const storeOptions = computed(() => store.state.balancerOptions.adjustSr);
    const adjustSr = ref(storeOptions);

    return { adjustSr };
  },
});
</script>
