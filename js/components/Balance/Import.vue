<template>
  <div class="form-file form-file-sm wf">
    <input
      type="file"
      id="importBalancerOptions"
      class="form-file-input d-none"
      @change="onChange"
    />
    <label for="importBalancerOptions" class="form-file-label w-100">
      <span class="form-file-button">
        <upload-icon />
      </span>
    </label>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

import UploadIcon from '@/components/svg/UploadIcon.vue';

export default defineComponent({
  name: 'Import',
  components: { UploadIcon },
  setup() {
    const store = useStore();

    const onReaderLoad = (event: ProgressEvent<FileReader>) => {
      if (!event.target) return;

      const source = event.target.result as string;
      const data = JSON.parse(source);

      if (data.format === 'xvb-1') {
        store.commit(MutationTypes.SET_BALANCER_OPTIONS, data.data);
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

label,
.form-file-button {
  background-color: #fff;
  border: 0;
  cursor: pointer;
}
</style>
