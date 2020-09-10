<template>
  <div class="modal fade" tabindex="-1" aria-hidden="true" ref="modalRef">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">{{ title }}</h5>
          <button type="button" class="close" data-dismiss="modal" aria-label="Close">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <slot />
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" data-dismiss="modal">
            Close
          </button>
          <button class="btn btn-primary" @click="$emit('save-changes')">
            Save changes
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, watch, onMounted, ref } from 'vue';

import Modal from '../../node_modules/bootstrap/js/src/modal';

export default defineComponent({
  name: 'Modal',
  props: {
    title: String,
    isActive: Boolean,
  },
  setup(props, { emit }) {
    const modalRef = ref(null);

    const setIsActive: (modal: Modal, isActive: boolean, isActiveP: boolean) => void = (
      modal,
      isActive,
      isActiveP
    ) => {
      if (isActive === isActiveP) {
        return;
      }

      if (isActive) {
        modal.show();
        return;
      }

      modal.hide();
    };

    let modal: null | Modal = null;
    onMounted(() => {
      modal = new Modal(modalRef.value);

      ((modalRef.value as unknown) as Element).addEventListener('hidden.bs.modal', () => {
        emit('close-modal');
      });

      setIsActive(modal, props.isActive || false, !props.isActive);
    });

    watch(
      () => props.isActive,
      (isActive, isActiveP) => setIsActive(modal, !!isActive, !!isActiveP)
    );

    return {
      modalRef,
    };
  },
});
</script>
