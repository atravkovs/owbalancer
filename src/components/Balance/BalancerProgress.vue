<template>
  <div class="mb-3">
    <div class="progress">
      <div
        class="progress-bar progress-bar-striped"
        :class="{
          'progress-bar-animated': current !== 1,
        }"
        role="progressbar"
        :aria-valuenow="Math.floor(current * 100)"
        aria-valuemin="0"
        aria-valuemax="100"
        :style="`width: ${Math.floor(current * 100)}%`"
      ></div>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';

import { useStore } from '@/store';

export default defineComponent({
  name: 'BalancerProgress',
  props: {
    progress: Object,
  },
  setup(props) {
    const store = useStore();
    const triesCount = computed(() => store.state.balancerOptions.triesCount);

    const current = computed(() =>
      props.progress ? props.progress.current / (props.progress.total * triesCount.value) : 0
    );

    return { current };
  },
});
</script>
