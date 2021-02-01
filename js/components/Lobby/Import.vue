<template>
  <div class="form-file form-file-sm wf">
    <input
      type="file"
      accept=".json"
      id="importFile"
      class="form-file-input d-none"
      ref="inp"
      @change="onChange"
    />
    <label for="importFile" class="form-file-label w-100">
      <span class="form-file-button">Import</span>
    </label>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

export default defineComponent({
  name: 'Import',
  setup() {
    const store = useStore();
    const inp = ref<HTMLInputElement | null>(null);

    const onReaderLoad = (event: ProgressEvent<FileReader>) => {
      if (!event.target) return;

      const source = event.target.result as string;

      try {
        const data = JSON.parse(source);

        if (inp?.value) {
          inp.value.value = '';
        }

        if (data.format === 'xv-1') {
          store.commit(MutationTypes.IMPORT_PLAYERS, data.players);
          return;
        }

        if (data.format_version === 9 && data.format_type === 'tournament') {
          store.commit(MutationTypes.IMPORT_PLAYERS_OLD, source);
          return;
        }

        throw new Error('Incorrect players export format');
      } catch (e) {
        // eslint-disable-next-line
        alert(`Format error: ${e.message}`);
      }
    };

    const onChange = (event: Event) => {
      const reader = new FileReader();
      const { files } = event.target as HTMLInputElement;

      console.log('hi');

      if (files !== null && files.length) {
        reader.onload = onReaderLoad;
        reader.readAsText(files[0]);
      }
    };

    return { inp, onChange };
  },
});
</script>

<style lang="scss" scoped>
.wf {
  width: 4rem;
}
</style>
