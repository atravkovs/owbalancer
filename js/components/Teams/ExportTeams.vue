<template>
  <dropdown id="exportTeams" title="Export">
    <drop-item @drop-click="exportText">Text</drop-item>
    <drop-item @drop-click="exportCSV">CSV</drop-item>
  </dropdown>
  <export-modal
    :isActive="isModalActive"
    :exportText="modalText"
    @close-modal="closeModal"
  />
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import Table from 'easy-table';
import { useStore } from '@/store';

import Dropdown from '@/components/Helpers/Dropdown.vue';
import DropItem from '@/components/Helpers/DropItem.vue';
import ExportModal from '@/components/Teams/ExportModal.vue';

export default defineComponent({
  name: 'ExportTeams',
  components: { Dropdown, DropItem, ExportModal },
  setup() {
    const modalText = ref('');
    const isModalActive = ref(false);
    const store = useStore();

    const exportText = () => {
      const { teams } = store.state;

      const text = teams.reduce((acc, team) => {
        const t = new Table();

        const teamAvgSr = Math.round(
          team.members.reduce(
            (accAvg, member) =>
              accAvg +
              store.state.players[member.uuid].stats.classes[member.role].rank,
            0
          ) / team.members.length
        );

        let teamText = `Team ${team.name} - ${teamAvgSr}\n=============================\n`;

        ['tank', 'dps', 'support'].forEach((role) => {
          team.members
            .filter((member) => member.role === role)
            .forEach((member) => {
              const { isSquire, isCaptain } = store.state.players[
                member.uuid
              ].identity;
              const { rank } = store.state.players[member.uuid].stats.classes[
                member.role
              ];
              const name = `${isSquire ? '⚔ ' : ''}${isCaptain ? '♛ ' : ''}${
                member.name
              }`;

              t.cell('Role', member.role);
              t.cell('Rank', Math.round(rank));
              t.cell('Name', name);
              t.newRow();
            });
        });

        teamText = `${teamText}${t.print()}`;

        return `${acc}\n\n${teamText}`;
      }, '');

      modalText.value = text;
      isModalActive.value = true;
    };

    const exportCSV = () => {
      const { teams } = store.state;
      let text = 'Team;Role;Rank;Name;Captain;Squire\n';

      const extendText = teams.reduce((acc, team) => {
        let teamText = '';
        ['tank', 'dps', 'support'].forEach((role) => {
          team.members
            .filter((member) => member.role === role)
            .forEach((member) => {
              const { isSquire, isCaptain } = store.state.players[
                member.uuid
              ].identity;
              teamText = `${teamText}"${team.name}";"${member.role}";"${
                member.rank
              }";"${member.name}";${isCaptain ? '1' : '0'};${
                isSquire ? '1' : '0'
              }\n`;
            });
        });

        return `${acc}${teamText}`;
      }, '');

      text = `${text}${extendText}`;
      modalText.value = text;
      isModalActive.value = true;
    };

    const closeModal = () => {
      isModalActive.value = false;
    };

    return { exportText, modalText, isModalActive, closeModal, exportCSV };
  },
});
</script>
