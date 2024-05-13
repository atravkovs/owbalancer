<template>
  <div class="form-control form-control-sm border-0 wf">
    <input
      type="file"
      accept=".json"
      id="importBalancerOptions"
      class="form-file-input d-none"
      ref="inp"
      @change="onChange"
    />
    <label for="importBalancerOptions" class="w-100">
      <span>
        <upload-icon />
      </span>
    </label>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

import UploadIcon from '@/components/svg/UploadIcon.vue';

export default defineComponent({
  name: 'Import',
  components: { UploadIcon },
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

        if (data.format === 'xvb-1') {
          store.commit(MutationTypes.SET_BALANCER_OPTIONS, data.data);
        } else {
          throw new Error('Incorrect balance options export format');
        }
      } catch (e) {
        // eslint-disable-next-line
        alert(`Format error: ${e.message}`);
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

    return { inp, onChange };
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
