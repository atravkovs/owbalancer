import { Teams } from '@/objects/team';
import { Players, ReservedPlayers } from '@/objects/player';

export type ArchiveEntry = {
  date: Date;
  name: string;
  teams: Teams;
  players: Players;
  reservedPlayers: ReservedPlayers;
};

export type Archive = ArchiveEntry[];
