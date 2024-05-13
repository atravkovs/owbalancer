<template>
  <modal
    title="Export"
    :isActive="isActive"
    @close-modal="$emit('close-modal')"
    variant="large"
    :hideAction="true"
  >
    <button @click="copyText" class="btn btn-outline-primary btn-lg mb-2">Copy</button>
    <textarea ref="tarea" :value="exportText"></textarea>
  </modal>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import Modal from '@/components/Helpers/Modal.vue';

export default defineComponent({
  name: 'ExportModal',
  components: { Modal },
  props: {
    isActive: Boolean,
    exportText: String,
  },
  setup() {
    const tarea = ref(null);

    const copyText = () => {
      const area = tarea.value as HTMLInputElement | null;

      if (!area) return;

      area.select();
      document.execCommand('copy');
    };

    return { tarea, copyText };
  },
});
</script>

<style lang="scss" scoped>
textarea {
  width: 100%;
  height: 70vh;
}
</style>
