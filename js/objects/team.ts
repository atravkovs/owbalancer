export type TeamMembers = {
    rank: number;
    uuid: string;
    name: string;
    role: 'dps' | 'support' | 'tank';
};

export type Team = {
    name: string;
    avgSr: number;
    totalSr: number;
    members: TeamMembers[];
};

export type Teams = Team[];

const createEmptyTeam: (tname: string) => Team = (name) => {
    return {
        name,
        avgSr: 0,
        totalSr: 0,
        members: [],
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
