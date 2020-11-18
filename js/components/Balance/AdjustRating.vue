<template>
  <div class="mb-3">
    <div class="form-check">
      <input type="checkbox" id="srScaling" class="form-check-input" v-model="adjustSr.isEnabled" />
      <label for="srScaling" class="form-check-label">Adjust player SR by main class</label>
    </div>
    <div class="d-flex justify-content-between">
      <div class="input-group mr-2">
        <input type="number" min="0" class="form-control" v-model.number="adjustSr.tank" />
        <span class="input-group-text">
          <role-icon rtype="tank" />
        </span>
        <span class="input-group-text">%</span>
      </div>
      <div class="input-group mr-2">
        <input type="number" min="0" class="form-control" v-model.number="adjustSr.dps" />
        <span class="input-group-text">
          <role-icon rtype="dps" />
        </span>
        <span class="input-group-text">%</span>
      </div>
      <div class="input-group">
        <input type="number" min="0" class="form-control" v-model.number="adjustSr.support" />
        <span class="input-group-text">
          <role-icon rtype="support" />
        </span>
        <span class="input-group-text">%</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from 'vue';

import { useStore } from '@/store';

import RoleIcon from '@/components/svg/RoleIcon.vue';

export default defineComponent({
  name: 'AdjustSr',
  components: { RoleIcon },
  setup() {
    const store = useStore();
    const storeOptions = computed(() => store.state.balancerOptions.adjustSr);
    const adjustSr = ref(storeOptions);

    return { adjustSr };
  },
});
</script>
