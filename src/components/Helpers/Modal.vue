<template>
  <div class="modal fade" tabindex="-1" aria-hidden="true" ref="modalRef">
    <div
      class="modal-dialog modal-dialog-centered"
      :class="[variant === 'large' ? 'modal-xl' : '', fullscreenClass]"
    >
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">{{ title }}</h5>
          <button
            type="button"
            class="btn-close"
            data-bs-dismiss="modal"
            aria-label="Close"
          ></button>
        </div>
        <div class="modal-body">
          <slot />
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
          <button v-if="!hideAction" class="btn btn-primary" @click="$emit('save-changes')">
            {{ customAction || 'Save changes' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, watch, onMounted, ref, computed } from 'vue';

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
    fullscreen: [String, Boolean],
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

    const fullscreenClass = computed(() => {
      const baseClass = 'modal-fullscreen';
      if (typeof props.fullscreen === 'string') {
        return `${baseClass}-${props.fullscreen}`;
      }
      if (typeof props.fullscreen === 'boolean') {
        return props.fullscreen ? baseClass : '';
      }
      return '';
    });

    return {
      modalRef,
      fullscreenClass,
    };
  },
});
</script>
