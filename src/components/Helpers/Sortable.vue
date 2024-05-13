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
  props: {
    handle: String,
  },
  setup(props, { emit }) {
    const container= ref<HTMLElement | null>(null);

    const onUpdate = (e: SortableEvent) => {
      emit('update-position', e);
    };

    onMounted(() => {
      if (container.value) {
        Sortable.create(container.value, {
          onUpdate,
          direction: 'vertical',
          handle: props.handle,
        });
      }
    });

    return { container };
  },
});
</script>
