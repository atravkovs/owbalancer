<template>
  <button @click="deletePlayers" type="button" class="btn btn-sm btn-danger">
    <svg
      width="1em"
      height="1em"
      viewBox="0 0 16 16"
      class="bi bi-trash-fill"
      fill="currentColor"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        fill-rule="evenodd"
        d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1H2.5zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5zM8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5zm3 .5a.5.5 0 0 0-1 0v7a.5.5 0 0 0 1 0v-7z"
      />
    </svg>
  </button>
</template>

<script lang="ts">
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'DeletePlayers',
  setup() {
    const store = useStore();

    const deletePlayers = () => {
      // eslint-disable-next-line
      if (window.confirm('Do you really want to Delete ALL players?'))
        store.commit(MutationTypes.DELETE_PLAYERS, undefined);
    };

    return { deletePlayers };
  },
});
</script>
