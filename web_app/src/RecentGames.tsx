import { useQuery } from "@tanstack/react-query";

type Game = {
	id: number;
	name: string;
	released: string | null;
	background_image: string | null;
};

async function getRecentGames(): Promise<Game[]> {
	const res = await fetch(
		`${import.meta.env.VITE_BACKEND_URL}/get_recent_games`,
	);

	if (!res.ok) throw new Error("Failed to fetch recent games");

	const data = await res.json();

	return data.results;
}

export default function RecentGames() {
	const {
		data: games = [],
		isLoading,
		error,
	} = useQuery({
		queryKey: ["recentGames"],
		queryFn: getRecentGames,
	});

	if (isLoading) return <p>Loading...</p>;
	if (error) return <p>Failed to load games.</p>;

	return (
		<div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
			{games.map((game) => (
				<GameCard key={game.id} {...game} />
			))}
		</div>
	);
}

function GameCard(props: Game) {
	return (
		<div className="flex flex-col border justify-center items-center gap-1 p-2 border-neutral-600">
			<img
				alt="background_image"
				src={props.background_image ?? undefined}
				className="w-24"
			/>
			<p>{props.name}</p>
			<p>{props.released}</p>
		</div>
	);
}
