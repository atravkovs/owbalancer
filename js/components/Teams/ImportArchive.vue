<template>
  <div class="form-file form-file-sm wf">
    <input type="file" id="importArchive" class="form-file-input d-none" @change="onChange" />
    <label for="importArchive" class="form-file-label w-100">
      <span class="form-file-button">Import</span>
    </label>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

export default defineComponent({
  name: 'ImportArchive',
  setup() {
    const store = useStore();

    const onReaderLoad = (event: ProgressEvent<FileReader>) => {
      if (!event.target) return;

      const source = event.target.result as string;
      const data = JSON.parse(source);

      if (data.format === 'xva-1') {
        store.commit(MutationTypes.IMPORT_ARCHIVE, data.data);
      }
    };

    const onChange = (event: Event) => {
      const reader = new FileReader();
      const { files } = event.target as HTMLInputElement;

      if (files !== null && files.length) {
        reader.onload = onReaderLoad;
        reader.readAsText(files[0]);
      }
    };

    return { onChange };
  },
});
</script>

<style lang="scss" scoped>
.wf {
  width: 4rem;
}
</style>
