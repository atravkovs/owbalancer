<template>
  <modal
    :isActive="player.identity.name.length > 0"
    :title="player.identity.name"
    @close-modal="closeModal"
    @save-changes="saveChanges"
  >
    <fieldset class="EditPlayer-Block">
      <h3>Identity</h3>
      <edit-identity :identity="player.identity" />
    </fieldset>
    <fieldset class="EditPlayer-Block">
      <h3>Stats</h3>
      <edit-stats
        :stats="player.stats"
        :uuid="player.identity.uuid"
        v-on:update-rank="updateRank"
        @update-specialization="updateSpecialization"
      />
    </fieldset>
  </modal>
</template>

<script lang="ts">
import { defineComponent, computed, ref } from 'vue';

import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';
import PlayerEditor from '@/objects/player';

import Modal from '@/components/Helpers/Modal.vue';
import EditStats from '@/components/Lobby/EditStats.vue';
import EditIdentity from '@/components/Lobby/EditIdentity.vue';

export default defineComponent({
  name: 'EditPlayer',
  components: { Modal, EditStats, EditIdentity },
  setup() {
    const store = useStore();
    const emptyPlayer = PlayerEditor.createDefaultPlayer('');

    const playerData = computed(() => store.state.players[store.state.editPlayer] || emptyPlayer);

    const player = ref(playerData);

    const closeModal = () => {
      store.commit(MutationTypes.EMPTY_NO_RANK, undefined);
      store.commit(MutationTypes.CLEAR_EDIT_PLAYER, undefined);
    };

    const saveChanges = () => {
      store.commit(MutationTypes.EMPTY_NO_RANK, undefined);
      // By executing mutation changes are being synchronized from object to local storage
      store.commit(MutationTypes.EDIT_PLAYER, player.value.identity.uuid);
    };

    // eslint-ignore-next-line
    const updateRank = (role: 'dps' | 'support' | 'tank', rank: number) => {
      store.commit(MutationTypes.EDIT_RANK, {
        role,
        rank,

        uuid: store.state.editPlayer,
      });
    };

    const updateSpecialization = (role: 'dps' | 'support' | 'tank', specialization: 'primary' | 'secondary', value: boolean) => {
      store.commit(MutationTypes.EDIT_SPECIALIZATION, {
        role,
        value,
        specialization,
        uuid: store.state.editPlayer,
      })
    };

    return { player, closeModal, saveChanges, updateRank, updateSpecialization };
  },
});
</script>

<style lang="scss" scoped>
.EditPlayer {
  &-Block:not(:last-child) {
    border-bottom: 1px solid black;
    padding-bottom: 1em;
    margin: 1em 0;
  }
}
</style>
