<template>
  <div class="row">
    <label for="playerFilter" class="col-2 col-form-label">Filter:</label>
    <div class="col-10">
      <input id="playerFilter" type="text" class="form-control form-control-sm" @input="filter" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { PlayerEntries } from '@/objects/player';

export default defineComponent({
  name: 'Filter',
  props: {
    players: Array as PropType<PlayerEntries>,
  },
  setup(props, { emit }) {
    const filter = (e: Event) => {
      const filterValue = (e.target as HTMLInputElement).value.toLowerCase();
      const filtered = props.players?.filter(([, p]) =>
        p.identity.name.toLowerCase().startsWith(filterValue)
      );
      emit('filter', filtered);
    };

    return { filter };
  },
});
</script>
