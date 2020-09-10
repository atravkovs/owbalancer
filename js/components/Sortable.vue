<template>
  <div ref="container">
    <slot />
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref } from 'vue';
import Sortable, { SortableEvent } from 'sortablejs';

export default defineComponent({
  name: 'Sortable',
  setup(_, { emit }) {
    const container = ref(null);

    const onUpdate = (e: SortableEvent) => {
      emit('update-position', e);
    };

    onMounted(() => {
      if (container.value) {
        Sortable.create(container.value, { onUpdate });
      }
    });

    return { container };
  },
});
</script>
