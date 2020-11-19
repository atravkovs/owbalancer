<template>
  <modal
    title="Archive"
    variant="large"
    :isActive="isActive"
    :hideAction="true"
    @close-modal="closeModal"
  >
    <div class="d-flex">
      <button class="btn btn-primary btn-sm mr-1" @click="saveCurrent">Save current</button>
      <import-archive />
    </div>
    <table class="table">
      <thead>
        <tr>
          <th scope="col">#</th>
          <th scope="col">Name</th>
          <th scope="col">Date</th>
          <th scope="col">Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(entry, i) in archive" :key="i">
          <td>{{ i + 1 }}</td>
          <td>{{ entry.name }}</td>
          <td>{{ entry.date }}</td>
          <td>
            <button class="btn btn-primary btn-sm" @click="() => select(i)">
              <cloud-up-icon />
            </button>
            <button class="btn btn-primary btn-sm ml-1" @click="() => download(i)">
              <download-icon />
            </button>
            <button class="btn btn-danger btn-sm ml-1" @click="() => removeFromArchive(i)">
              <bin-icon />
            </button>
          </td>
        </tr>
      </tbody>
    </table>
  </modal>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';

import { useStore } from '@/store';
import Utils from '@/utils';

import BinIcon from '@/components/svg/BinIcon.vue';
import Modal from '@/components/Helpers/Modal.vue';
import MutationTypes from '@/store/mutation-types';
import CloudUpIcon from '@/components/svg/CloudUpIcon.vue';
import DownloadIcon from '@/components/svg/DownloadIcon.vue';
import ImportArchive from '@/components/Teams/ImportArchive.vue';

export default defineComponent({
  name: 'Archive',
  components: { Modal, BinIcon, CloudUpIcon, DownloadIcon, ImportArchive },
  setup() {
    const store = useStore();
    const isActive = computed(() => store.state.isArchive);
    const archive = computed(() => store.state.archive);

    const closeModal = () => {
      store.commit(MutationTypes.TOGGLE_ARCHIVE, undefined);
    };

    const saveCurrent = () => {
      store.commit(MutationTypes.SAVE_TO_ARCHIVE, undefined);
    };

    const removeFromArchive = (id: number) => {
      store.commit(MutationTypes.REMOVE_FROM_ARCHIVE, id);
    };

    const select = (id: number) => {
      store.commit(MutationTypes.SELECT_ARCHIVE, id);
    };

    const download = (id: number) => {
      const exportData = {
        format: 'xva-1',
        data: archive.value[id],
      };

      Utils.download(`archive-${archive.value[id].name}`, JSON.stringify(exportData));
    };

    return { archive, isActive, closeModal, saveCurrent, removeFromArchive, select, download };
  },
});
</script>
