import { v4 as uuidv4 } from 'uuid';
import { Entries } from '@/objects/common';

export type Identity = {
  uuid: string;
  name: string;
  isLocked: boolean;
  isSquire: boolean;
  isCaptain: boolean;
  isFullFlex: boolean;
};

export type ClassType = {
  rank: number;
  priority: number;
  isActive: boolean;
  primary: boolean;
  secondary: boolean;
};

export type DescribedClassType = ClassType & { role: string };

export type Classes = {
  dps: ClassType;
  tank: ClassType;
  support: ClassType;
};

export type Stats = {
  classes: Classes;
};

export type Player = {
  identity: Identity;
  stats: Stats;
  createdAt: Date;
};

export type Players = {
  [tag: string]: Player;
};

export type LobbyType = 'players' | 'backup';

export type PlayerEntries = Entries<Players>;

export type ReservedPlayers = string[];

const createDefaultPlayer: (name: string) => Player = name => {
  return {
    identity: {
      name,
      uuid: uuidv4(),
      isLocked: false,
      isCaptain: false,
      isSquire: false,
      isFullFlex: false,
    },
    stats: {
      classes: {
        dps: {
          rank: 0,
          playHours: 0,
          priority: 0,
          primary: false,
          isActive: false,
          secondary: false,
        },
        tank: {
          rank: 0,
          playHours: 0,
          priority: 1,
          primary: false,
          isActive: false,
          secondary: false,
        },
        support: {
          rank: 0,
          playHours: 0,
          priority: 2,
          primary: false,
          isActive: false,
          secondary: false,
        },
      },
    },
    createdAt: new Date(),
  };
};

const getRole = (roles: Classes, role: string): ClassType => {
  if (role === 'dps') {
    return roles.dps;
  }

  if (role === 'tank') {
    return roles.tank;
  }

  return roles.support;
};

const getTopRole = (player: Player): ClassType => {
  return Object.values(player.stats.classes)
    .filter(role => role.isActive)
    .sort((a, b) => a.priority - b.priority)[0];
};

const getTopRoleName = (player: Player): string => {
  return Object.entries(player.stats.classes)
    .filter(([, role]) => role.isActive)
    .sort(([, a], [, b]) => a.priority - b.priority)[0][0];
};

const getTopRank = (player: Player): number => {
  const topRole = getTopRole(player);
  return topRole ? topRole.rank : 0;
};

export default {
  getRole,
  getTopRank,
  getTopRole,
  getTopRoleName,
  createDefaultPlayer,
};
