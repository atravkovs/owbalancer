<template>
  <div class="dropdown">
    <button
      class="btn btn-sm btn-secondary dropdown-toggle"
      :id="`dropdown_${id}`"
      data-toggle="dropdown"
      aria-expanded="false"
      ref="dropdownRef"
    >
      <slot name='title'>{{ title }}</slot>
    </button>
    <ul class="dropdown-menu" :aria-labelledby="`dropdown_${id}`">
      <slot />
    </ul>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref } from 'vue';

import Dropdown from 'bootstrap/js/src/dropdown';

export default defineComponent({
  name: 'Dropdown',
  props: {
    id: String,
    title: String,
  },
  setup() {
    const dropdownRef = ref(null);

    onMounted(() => {
      // eslint-disable-next-line
      const a = new Dropdown(dropdownRef.value);
    });

    return { dropdownRef };
  },
});
</script>
