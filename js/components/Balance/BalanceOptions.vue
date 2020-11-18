<template>
  <div class="mb-3">
    <label class="form-label">Options</label>
    <div class="form-check">
      <input
        type="checkbox"
        id="lowRankLimiter"
        class="form-check-input"
        v-model="options.lowRankLimiter"
      />
      <label for="lowRankLimiter" class="form-check-label">Low rank limiter</label>
    </div>
    <div class="form-check">
      <input
        type="checkbox"
        id="disallowSecondaryRoles"
        class="form-check-input"
        v-model="options.disallowSecondaryRoles"
      />
      <label for="disallowSecondaryRoles" class="form-check-label"
        >Disallow duplicate secondary roles</label
      >
    </div>
    <div class="form-check">
      <input
        type="checkbox"
        id="dispersionMiminizer"
        class="form-check-input"
        v-model="options.dispersionMiminizer"
      />
      <label for="dispersionMiminizer" class="form-check-label">Enable dispersion minimizer</label>
    </div>
  </div>
</template>

<script lang="ts">
import { useStore } from '@/store';
import { computed, defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'BalanceOptions',
  setup() {
    const store = useStore();
    const storeOptions = computed(() => store.state.balancerOptions);
    const options = ref(storeOptions);

    return { options };
  },
});
</script>
