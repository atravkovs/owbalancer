<template>
  <Sortable v-on:update-position="updatePosition" handle=".sortable-handler">
    <EditRole
      v-for="role in roles"
      :key="role.role"
      :rtype="role.role"
      :role="playerStats.classes[role.role]"
    />
  </Sortable>
</template>

<script lang="ts">
import { defineComponent, ref, PropType, computed } from 'vue';
import { SortableEvent } from 'sortablejs';

import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

import {
  Classes,
  ClassType,
  DescribedClassType,
  Stats,
} from '@/objects/player';

import Sortable from '@/components/Helpers/Sortable.vue';
import EditRole from '@/components/Lobby/EditRole.vue';

export default defineComponent({
  name: 'EditStats',
  props: {
    uuid: String,
    stats: Object as PropType<Stats>,
  },
  components: { Sortable, EditRole },
  setup(props) {
    const store = useStore();
    const stats = computed(() => props.stats);
    const uuid = computed(() => props.uuid);
    const result: DescribedClassType[] = [];
    const playerStats = ref(stats);

    const roles = computed(() =>
      Object.entries(stats?.value?.classes || {})
        .reduce((acc, [role, v]) => {
          return [...acc, { role, ...v }];
        }, result)
        .sort((a, b) => a.priority - b.priority)
    );

    const getRole = (classesList: Classes, role: string): ClassType => {
      if (role === 'tank') return classesList.tank;
      if (role === 'support') return classesList.support;

      return classesList.dps;
    };

    const updatePosition = ({ oldIndex, newIndex }: SortableEvent) => {
      if (
        oldIndex === undefined ||
        newIndex === undefined ||
        oldIndex === newIndex ||
        uuid.value === undefined ||
        !stats.value
      )
        return;

      const directionUp = newIndex - oldIndex < 0;
      if (!playerStats.value) return;

      const classes = { ...playerStats.value.classes };

      Object.keys(classes).forEach((role) => {
        const sRole = getRole(classes, role);

        if (sRole.priority === oldIndex) {
          sRole.priority = newIndex;
          return;
        }

        if (
          directionUp &&
          sRole.priority < oldIndex &&
          sRole.priority >= newIndex
        ) {
          sRole.priority += 1;
        }

        if (
          !directionUp &&
          sRole.priority > oldIndex &&
          sRole.priority <= newIndex
        ) {
          sRole.priority -= 1;
        }
      });

      store.commit(MutationTypes.UPDATE_STATS, {
        uuid: uuid.value,
        stats: { rank: stats.value.rank, classes },
      });
    };

    return { playerStats, roles, updatePosition };
  },
});
</script>
