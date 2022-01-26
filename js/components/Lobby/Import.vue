<template>
  <div class="form-control form-control-sm rounded-0 rounded-end wf">
    <input
      type="file"
      accept=".json"
      :id="`importFile${lobby}`"
      class="form-file-input d-none"
      ref="inp"
      @change="onChange"
    />
    <label :for="`importFile${lobby}`" class="w-100">
      <span>Import</span>
    </label>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType, ref } from 'vue';
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';
import { LobbyType } from '@/objects/player';

export default defineComponent({
  name: 'Import',
  props: {
    lobby: {
      type: String as PropType<LobbyType>,
      default: 'players',
    }
  },
  setup(props) {
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
          store.commit(MutationTypes.IMPORT_PLAYERS, { players: data.players, lobby: props.lobby });
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
@import '~bootstrap/scss/functions';
@import '~bootstrap/scss/variables';

.wf {
  width: 4rem;
}
.form-control {
  background-color: $gray-200;
}
</style>
