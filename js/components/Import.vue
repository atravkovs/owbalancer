<template>
  <div class="form-file form-file-sm">
    <input type="file" id="importFile" class="form-file-input" @change="onChange" />
    <label for="importFile" class="form-file-label">
      <span class="form-file-text">Choose file...</span>
      <span class="form-file-button">Import</span>
    </label>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

export default defineComponent({
  name: 'Import',
  setup() {
    const store = useStore();

    const onReaderLoad = (event: ProgressEvent<FileReader>) => {
      if (!event.target) return;

      const players = JSON.parse(event.target.result as string);
      store.commit(MutationTypes.IMPORT_PLAYERS, players);
    };

    const onChange = (event: Event) => {
      const reader = new FileReader();
      const { files } = event.target as HTMLInputElement;

      if (files !== null && files.length) {
        reader.onload = onReaderLoad;
        reader.readAsText(files[0]);
      }

      console.log('Hi!');
    };

    return { onChange };
  },
});
</script>
