<template>
  <div class="btn-group-vertical">
    <button title="Add all" @click="addAll" class="btn btn-light">
      &gt;&gt;
    </button>
    <button
      title="Add selected (Ctrl + LClick)"
      @click="addSelected"
      class="btn btn-light"
    >
      &gt;
    </button>
    <button
      title="Backup selected (Ctrl + LClick)"
      @click="backupSelected"
      class="btn btn-light"
    >
      &lt;
    </button>
    <button title="Backup all" @click="backupAll" class="btn btn-light">
      &lt;&lt;
    </button>
  </div>
</template>

<script lang="ts">
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'Swap',
  setup() {
    const store = useStore();

    const backupAll = () => {
      store.commit(MutationTypes.ADD_PLAYERS, {
        players: store.state.players,
        lobby: 'backup',
      });
    };

    const backupSelected = () => {
      const selectedPlayers = Object.entries(
        store.state.selectPlayers.players
      )
        .filter(([, value]) => value)
        .map(([playerId]) => playerId);

      const addPlayers = Object.entries(store.state.players)
        .filter(([uuid]) => selectedPlayers.includes(uuid))
        .reduce((acc, [uuid, player]) => ({ ...acc, [uuid]: player }), {});

      store.commit(MutationTypes.ADD_PLAYERS, {
        players: addPlayers,
        lobby: 'backup',
      });

      store.commit(MutationTypes.SELECT_PLAYERS, {
        playerIds: selectedPlayers,
        lobby: 'players',
      });
    };

    const addSelected = () => {
      const selectedPlayers = Object.entries(
        store.state.selectPlayers.backup
      )
        .filter(([, value]) => value)
        .map(([playerId]) => playerId);

      const addPlayers = Object.entries(store.state.backup)
        .filter(([uuid]) => selectedPlayers.includes(uuid))
        .reduce((acc, [uuid, player]) => ({ ...acc, [uuid]: player }), {});

      console.log('Add Players', JSON.stringify(addPlayers));
      

      store.commit(MutationTypes.ADD_PLAYERS, {
        players: addPlayers,
        lobby: 'players',
      });

      store.commit(MutationTypes.SELECT_PLAYERS, {
        playerIds: selectedPlayers,
        lobby: 'backup',
      });

      
      if (store.state.teams.length > 0) {
        selectedPlayers.forEach(player => {
          store.commit(MutationTypes.ADD_RESERVE, player);
        });
      }
    };

    const addAll = () => {
      store.commit(MutationTypes.ADD_PLAYERS, {
        players: store.state.backup,
        lobby: 'players',
      });

      
      if (store.state.teams.length > 0) {
        Object.keys(store.state.backup).forEach(player => {
          store.commit(MutationTypes.ADD_RESERVE, player);
        });
      }
    };

    return { backupAll, backupSelected, addSelected, addAll };
  },
});
</script>

