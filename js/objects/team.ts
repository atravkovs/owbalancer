import { v4 as uuidv4 } from 'uuid';
import NameGenerator from 'naampje';

export type TeamMembers = {
    rank: number;
    uuid: string;
    name: string;
    primary: boolean;
    secondary: boolean;
    role: 'dps' | 'support' | 'tank';
};

export type Team = {
    uuid: string;
    name: string;
    avgSr: number;
    totalSr: number;
    members: TeamMembers[];
};

export type Teams = Team[];

const createEmptyTeam: () => Team = () => {
    const name = NameGenerator.name();
    return {
        name,
        avgSr: 0,
        totalSr: 0,
        members: [],
        uuid: uuidv4(),
    };
};

const calcStats: (team: Team) => { avgSr: number; totalSr: number } = (team) => {
    const totalSr = team.members.reduce((acc, member) => acc + member.rank, 0);
    const avgSr = totalSr / team.members.length;

    return { avgSr, totalSr };
};

export default {
    calcStats,
    createEmptyTeam,
};
