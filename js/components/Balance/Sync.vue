<template>
  <div class="position-absolute d-flex">
    <button class="btn btn-sm" @click="download">
      <download-icon />
    </button>
    <import />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';

import Utils from '@/utils';
import Import from '@/components/Balance/Import.vue';
import DownloadIcon from '@/components/svg/DownloadIcon.vue';
import { useStore } from '@/store';

export default defineComponent({
  name: 'Sync',
  components: { Import, DownloadIcon },
  setup() {
    const store = useStore();
    const options = computed(() => store.state.balancerOptions);

    const download = () => {
      const data = options.value;
      const exportData = {
        format: 'xvb-1',
        data,
      };

      Utils.download(
        `balancerOptions-${new Date().toLocaleString('ru-RU')}.json`,
        JSON.stringify(exportData)
      );
    };

    return { download };
  },
});
</script>

<style scoped>
.position-absolute {
  right: 0;
  top: 0;
}
</style>
