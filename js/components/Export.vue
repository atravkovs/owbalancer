<template>
  <button @click="onClick" class="btn btn-secondary btn-sm">Export</button>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';
import { useStore } from '@/store';

function download(filename: string, text: string) {
  const element = document.createElement('a');
  element.setAttribute(
    'href',
    `data:application/json;charset=utf-8,${encodeURIComponent(text)}`
  );
  element.setAttribute('download', filename);

  element.style.display = 'none';
  document.body.appendChild(element);

  element.click();

  document.body.removeChild(element);
}

export default defineComponent({
  name: 'Export',
  setup() {
    const store = useStore();
    const players = computed(() => store.state.players);

    const onClick = () => {
      const data = players.value;
      download(
        `balancer-${new Date().toLocaleString('ru-RU')}.json`,
        JSON.stringify(data)
      );
    };

    return { onClick };
  },
});
</script>
