<template>
  <div class="modal fade" tabindex="-1" aria-hidden="true" ref="modalRef">
    <div class="modal-dialog modal-dialog-centered" :class="{ 'modal-xl': variant === 'large' }">
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
          <button class="btn btn-secondary" data-dismiss="modal">Close</button>
          <button v-if="!hideAction" class="btn btn-primary" @click="$emit('save-changes')">
            {{ customAction || 'Save changes' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, watch, onMounted, ref } from 'vue';

import { Modal as ModalType } from 'bootstrap';
import Modal from 'bootstrap/js/src/modal';


export default defineComponent({
  name: 'Modal',
  props: {
    title: String,
    variant: String,
    isActive: Boolean,
    hideAction: Boolean,
    customAction: String,
  },
  setup(props, { emit }) {
    const modalRef = ref(null);

    const setIsActive: (modal: ModalType | null, isActive: boolean, isActiveP: boolean) => void = (
      modal,
      isActive,
      isActiveP
    ) => {
      if (!modal) return;

      if (isActive === isActiveP) {
        return;
      }

      if (isActive) {
        modal.show();
        return;
      }

      modal.hide();
    };

    let modal: null | ModalType = null;
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
