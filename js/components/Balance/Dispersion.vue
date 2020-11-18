<template>
  <div class="mb-3">
    <label for="dispersionRange" class="form-label">
      Dispersion:
      <b>±{{ options.range }}</b>
    </label>
    <input
      type="range"
      class="form-range"
      min="0"
      max="120"
      step="1"
      v-model.number="options.range"
    />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from 'vue';
import { useStore } from '@/store';

export default defineComponent({
  name: 'Dispersion',
  setup() {
    const store = useStore();
    const storeOptions = computed(() => store.state.balancerOptions);
    const options = ref(storeOptions);

    return { options };
  },
});
</script>

<style lang="scss" scoped>
input[type='range'] {
  &::-webkit-slider-runnable-track {
    background: linear-gradient(to right, #33ccff 0%, #ff0000 100%);
  }
}
</style>
