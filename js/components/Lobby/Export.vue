<template>
  <button @click="onClick" class="btn btn-secondary btn-sm rounded-0 rounded-start">Export</button>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';
import { useStore } from '@/store';
import { Players } from '@/objects/player';
import Utils from '@/utils';

type ExportData = {
  format: string;
  players: Players;
};

export default defineComponent({
  name: 'Export',
  setup() {
    const store = useStore();
    const players = computed(() => store.state.players);

    const onClick = () => {
      const data = players.value;
      const exportData: ExportData = {
        format: 'xv-1',
        players: data,
      };

      Utils.download(
        `players-${new Date().toLocaleString('ru-RU')}.json`,
        JSON.stringify(exportData)
      );
    };

    return { onClick };
  },
});
</script>
