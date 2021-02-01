<template>
  <div class="AdjustRole d-flex mx-auto">
    <div>
      <button
        @click="() => changeActive('any')"
        class="btn btn-sm btn-primary mr-1 mb-1"
        :class="{ active: 'any' === active }"
      >
        <specification-icon specification="any" :rtype="role" />
      </button>
      <button
        @click="() => changeActive('primary')"
        class="btn btn-sm btn-primary mr-1 mb-1"
        :class="{ active: 'primary' === active }"
      >
        <specification-icon specification="primary" :rtype="role" />
      </button>
      <button
        @click="() => changeActive('secondary')"
        class="btn btn-sm btn-primary mr-1"
        :class="{ active: 'secondary' === active }"
      >
        <specification-icon specification="secondary" :rtype="role" />
      </button>
    </div>
    <div>
      <bezier v-model="adjustSr[role][active]" />
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, PropType, ref } from 'vue';
import { useStore } from '@/store';

import Bezier from '@/components/Helpers/Bezier.vue';
import SpecificationIcon from '../svg/SpecificationIcon.vue';

export default defineComponent({
  name: 'AdjustRole',
  components: { Bezier, SpecificationIcon },
  props: {
    role: String as PropType<'tank' | 'support' | 'dps'>,
  },
  setup() {
    const active = ref<'any' | 'primary' | 'secondary'>('any');
    const store = useStore();
    const storeOptions = computed(() => store.state.balancerOptions.adjustSr);
    const adjustSr = ref(storeOptions);

    const changeActive = (to: 'any' | 'primary' | 'secondary') => {
      active.value = to;
    };

    return { active, adjustSr, changeActive };
  },
});
</script>

<style lang="scss">
.AdjustRole {
  width: min-content;
}
</style>
